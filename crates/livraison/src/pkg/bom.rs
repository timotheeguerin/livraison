//! A minimal writer for the macOS `BOMStore` "Bill of Materials" format.
//!
//! This is a from-scratch port of the algorithm used by `bomutils`' `mkbom`
//! (<https://github.com/hogliux/bomutils>), producing a byte-compatible BOM that
//! `lsbom` and the macOS `installer` accept. All multi-byte integers are stored
//! big-endian.
//!
//! The BOM records, for every path in the payload (the root `.`, every
//! intermediate directory and every file), its type, permission mode, owner,
//! size and a CRC-32 checksum of the file contents.

use std::collections::BTreeMap;

/// Fixed header size mkbom reserves (the header itself is only 32 bytes; the
/// remainder is zero padding).
const HEADER_SIZE: u32 = 512;

const TYPE_FILE: u8 = 1;
const TYPE_DIR: u8 = 2;

/// Compute the POSIX `cksum` CRC-32 (polynomial `0x04C11DB7`, MSB-first, with
/// the message length appended) of `data`. This is the checksum macOS records
/// in the BOM for each file — it is **not** the common IEEE/zlib CRC-32.
fn cksum_crc32(data: &[u8]) -> u32 {
    const POLY: u32 = 0x04C1_1DB7;
    // Build the lookup table (non-reflected).
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = (i as u32) << 24;
        let mut k = 0;
        while k < 8 {
            c = if c & 0x8000_0000 != 0 {
                (c << 1) ^ POLY
            } else {
                c << 1
            };
            k += 1;
        }
        table[i] = c;
        i += 1;
    }

    let mut crc: u32 = 0;
    for &b in data {
        crc = (crc << 8) ^ table[(((crc >> 24) as u8) ^ b) as usize];
    }
    // Append the length, least-significant byte first.
    let mut len = data.len();
    while len != 0 {
        crc = (crc << 8) ^ table[(((crc >> 24) as u8) ^ (len as u8)) as usize];
        len >>= 8;
    }
    !crc
}

/// A single entry (path) to record in the BOM.
#[derive(Debug, Clone)]
pub struct BomFile {
    /// Permission mode including the file-type bits (e.g. `0o100755`, `0o40755`).
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub mtime: u32,
    /// File size in bytes (0 for directories).
    pub size: u32,
    /// CRC-32 (IEEE) checksum of the contents (0 for directories).
    pub checksum: u32,
}

#[derive(Default)]
struct Node {
    children: BTreeMap<String, Node>,
    file: Option<BomFile>,
    is_dir: bool,
}

/// Builds the in-memory path tree that mirrors mkbom's `Node` hierarchy.
#[derive(Default)]
pub struct BomBuilder {
    root: Node,
}

impl BomBuilder {
    pub fn new() -> Self {
        BomBuilder::default()
    }

    /// Add a regular file at an absolute payload path (e.g.
    /// `/usr/local/bin/foo`). All intermediate directories are created
    /// automatically.
    pub fn add_file(&mut self, dest: &str, mode: u32, uid: u32, gid: u32, mtime: u32, data: &[u8]) {
        let file = BomFile {
            mode: with_type(mode, false),
            uid,
            gid,
            mtime,
            size: data.len() as u32,
            checksum: cksum_crc32(data),
        };
        self.insert(dest, Some((file, mtime, uid, gid)));
    }

    fn insert(&mut self, dest: &str, file: Option<(BomFile, u32, u32, u32)>) {
        // The tree is rooted at a synthetic node whose single child is ".",
        // matching the structure `mkbom` produces (the root "." is itself a
        // recorded path).
        let mut components: Vec<&str> = vec!["."];
        components.extend(dest.split('/').filter(|c| !c.is_empty()));
        let mut node = &mut self.root;
        let last = components.len().saturating_sub(1);
        for (i, comp) in components.iter().enumerate() {
            let child = node.children.entry((*comp).to_string()).or_default();
            if i == last {
                if let Some((f, _, _, _)) = &file {
                    child.file = Some(f.clone());
                    child.is_dir = false;
                }
            } else {
                child.is_dir = true;
            }
            node = child;
        }
    }
}

/// Force (or clear) the regular-file / directory type bits on a mode.
fn with_type(mode: u32, is_dir: bool) -> u32 {
    let base = mode & 0o7777;
    if is_dir {
        0o040000 | base
    } else {
        0o100000 | base
    }
}

/// Accumulates the serialized blocks and named variables of a BOM.
struct Store {
    /// Data blocks. Index 0 is the reserved null block.
    blocks: Vec<Vec<u8>>,
    /// Named variables, in insertion order: `(name, block_id)`.
    vars: Vec<(String, u32)>,
}

impl Store {
    fn new() -> Self {
        Store {
            blocks: vec![Vec::new()], // null block 0
            vars: Vec::new(),
        }
    }

    fn add_block(&mut self, data: Vec<u8>) -> u32 {
        let id = self.blocks.len() as u32;
        self.blocks.push(data);
        id
    }

    fn add_var(&mut self, name: &str, data: Vec<u8>) {
        let id = self.add_block(data);
        self.vars.push((name.to_string(), id));
    }

    fn serialize(&self) -> Vec<u8> {
        // --- vars region ---
        let mut vars = Vec::new();
        vars.extend_from_slice(&(self.vars.len() as u32).to_be_bytes());
        for (name, block_id) in &self.vars {
            vars.extend_from_slice(&block_id.to_be_bytes());
            vars.push(name.len() as u8);
            vars.extend_from_slice(name.as_bytes());
        }

        // --- entries region (all non-null blocks concatenated) ---
        let mut entries = Vec::new();
        let mut offsets = vec![0u32; self.blocks.len()];
        for (i, block) in self.blocks.iter().enumerate() {
            if i == 0 {
                continue;
            }
            offsets[i] = entries.len() as u32;
            entries.extend_from_slice(block);
        }

        let size_of_vars = vars.len() as u32;
        let entry_size = entries.len() as u32;
        let base = HEADER_SIZE + size_of_vars;

        // --- block table ---
        let mut block_table = Vec::new();
        block_table.extend_from_slice(&(self.blocks.len() as u32).to_be_bytes());
        for (i, block) in self.blocks.iter().enumerate() {
            if i == 0 {
                block_table.extend_from_slice(&0u32.to_be_bytes()); // address
                block_table.extend_from_slice(&0u32.to_be_bytes()); // length
            } else {
                block_table.extend_from_slice(&(base + offsets[i]).to_be_bytes());
                block_table.extend_from_slice(&(block.len() as u32).to_be_bytes());
            }
        }

        // --- free list (empty, but two zero pointers like mkbom) ---
        let mut free_list = Vec::new();
        free_list.extend_from_slice(&0u32.to_be_bytes());
        for _ in 0..2 {
            free_list.extend_from_slice(&0u32.to_be_bytes());
            free_list.extend_from_slice(&0u32.to_be_bytes());
        }

        let index_offset = HEADER_SIZE + size_of_vars + entry_size;
        let index_length = block_table.len() as u32 + free_list.len() as u32;

        // --- header ---
        let mut header = vec![0u8; HEADER_SIZE as usize];
        header[0..8].copy_from_slice(b"BOMStore");
        header[8..12].copy_from_slice(&1u32.to_be_bytes()); // version
        header[12..16].copy_from_slice(&(self.blocks.len() as u32 - 1).to_be_bytes()); // numberOfBlocks
        header[16..20].copy_from_slice(&index_offset.to_be_bytes());
        header[20..24].copy_from_slice(&index_length.to_be_bytes());
        header[24..28].copy_from_slice(&HEADER_SIZE.to_be_bytes()); // varsOffset
        header[28..32].copy_from_slice(&size_of_vars.to_be_bytes()); // varsLength

        let mut out = Vec::new();
        out.extend_from_slice(&header);
        out.extend_from_slice(&vars);
        out.extend_from_slice(&entries);
        out.extend_from_slice(&block_table);
        out.extend_from_slice(&free_list);
        out
    }
}

fn tree_block(child: u32, block_size: u32, path_count: u32) -> Vec<u8> {
    let mut b = Vec::with_capacity(21);
    b.extend_from_slice(b"tree");
    b.extend_from_slice(&1u32.to_be_bytes()); // version
    b.extend_from_slice(&child.to_be_bytes());
    b.extend_from_slice(&block_size.to_be_bytes());
    b.extend_from_slice(&path_count.to_be_bytes());
    b.push(0); // unknown3
    b
}

fn empty_paths_block() -> Vec<u8> {
    let mut b = Vec::with_capacity(12);
    b.extend_from_slice(&1u16.to_be_bytes()); // isLeaf
    b.extend_from_slice(&0u16.to_be_bytes()); // count
    b.extend_from_slice(&0u32.to_be_bytes()); // forward
    b.extend_from_slice(&0u32.to_be_bytes()); // backward
    b
}

/// Serialize the accumulated tree into BOM bytes.
pub fn build(builder: &BomBuilder) -> Vec<u8> {
    // Count all nodes (every path, including the root ".").
    let num = count_nodes(&builder.root);
    let mut store = Store::new();

    // --- BomInfo ---
    let mut info = Vec::new();
    info.extend_from_slice(&1u32.to_be_bytes()); // version
    info.extend_from_slice(&(num + 1).to_be_bytes()); // numberOfPaths
    let info_entries: u32 = if num != 0 { 1 } else { 0 };
    info.extend_from_slice(&info_entries.to_be_bytes());
    if num != 0 {
        info.extend_from_slice(&[0u8; 16]); // one zeroed BOMInfoEntry
    }
    store.add_var("BomInfo", info);

    // --- Paths tree ---
    // Breadth-first walk assigning each node an id (j+1). Because our payloads
    // are small (<256 paths) everything fits in a single leaf block, matching
    // the `num_paths == 1` branch in mkbom.
    let mut leaf_indices: Vec<(u32, u32)> = Vec::new(); // (index0 -> PathInfo1, index1 -> BOMFile)
    let mut queue: std::collections::VecDeque<(u32, &Node)> = std::collections::VecDeque::new();
    queue.push_back((0, &builder.root));
    let mut j: u32 = 0;
    while let Some((parent, node)) = queue.pop_front() {
        for (name, child) in &node.children {
            let id = j + 1;

            // BOMPathInfo2
            let (ftype, mode, uid, gid, mtime, size, checksum) = if let Some(f) = &child.file {
                (
                    TYPE_FILE,
                    f.mode as u16,
                    f.uid,
                    f.gid,
                    f.mtime,
                    f.size,
                    f.checksum,
                )
            } else {
                (TYPE_DIR, with_type(0o0755, true) as u16, 0, 0, 0, 0, 0)
            };
            let _ = child.is_dir;
            let mut info2 = Vec::with_capacity(31);
            info2.push(ftype);
            info2.push(1); // unknown0
            info2.extend_from_slice(&3u16.to_be_bytes()); // architecture
            info2.extend_from_slice(&mode.to_be_bytes());
            info2.extend_from_slice(&uid.to_be_bytes());
            info2.extend_from_slice(&gid.to_be_bytes());
            info2.extend_from_slice(&mtime.to_be_bytes());
            info2.extend_from_slice(&size.to_be_bytes());
            info2.push(1); // unknown1
            info2.extend_from_slice(&checksum.to_be_bytes());
            info2.extend_from_slice(&0u32.to_be_bytes()); // linkNameLength
            let info2_id = store.add_block(info2);

            // BOMPathInfo1 { id, index -> info2 }
            let mut info1 = Vec::with_capacity(8);
            info1.extend_from_slice(&id.to_be_bytes());
            info1.extend_from_slice(&info2_id.to_be_bytes());
            let info1_id = store.add_block(info1);

            // BOMFile { parent, name\0 }
            let mut file_block = Vec::with_capacity(5 + name.len());
            file_block.extend_from_slice(&parent.to_be_bytes());
            file_block.extend_from_slice(name.as_bytes());
            file_block.push(0);
            let file_id = store.add_block(file_block);

            leaf_indices.push((info1_id, file_id));
            queue.push_back((id, child));
            j += 1;
        }
    }

    // Single leaf BOMPaths block holding every path index.
    let mut leaf = Vec::new();
    leaf.extend_from_slice(&1u16.to_be_bytes()); // isLeaf
    leaf.extend_from_slice(&(leaf_indices.len() as u16).to_be_bytes()); // count
    leaf.extend_from_slice(&0u32.to_be_bytes()); // forward
    leaf.extend_from_slice(&0u32.to_be_bytes()); // backward
    for (i0, i1) in &leaf_indices {
        leaf.extend_from_slice(&i0.to_be_bytes());
        leaf.extend_from_slice(&i1.to_be_bytes());
    }
    let leaf_id = store.add_block(leaf);
    let paths_tree = tree_block(leaf_id, 4096, num);
    store.add_var("Paths", paths_tree);

    // --- HLIndex / VIndex / Size64 (all empty trees) ---
    let hl_leaf = store.add_block(empty_paths_block());
    store.add_var("HLIndex", tree_block(hl_leaf, 4096, 0));

    let v_leaf = store.add_block(empty_paths_block());
    let v_tree = tree_block(v_leaf, 128, 0);
    let v_tree_id = store.add_block(v_tree);
    let mut vindex = Vec::with_capacity(13);
    vindex.extend_from_slice(&1u32.to_be_bytes()); // unknown0
    vindex.extend_from_slice(&v_tree_id.to_be_bytes()); // indexToVTree
    vindex.extend_from_slice(&0u32.to_be_bytes()); // unknown2
    vindex.push(0); // unknown3
    store.add_var("VIndex", vindex);

    let sz_leaf = store.add_block(empty_paths_block());
    store.add_var("Size64", tree_block(sz_leaf, 4096, 0));

    store.serialize()
}

fn count_nodes(node: &Node) -> u32 {
    let mut n = 0;
    for child in node.children.values() {
        n += 1 + count_nodes(child);
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cksum_matches_posix_cksum() {
        // Values verified against the system `cksum` / `mkbom`.
        assert_eq!(cksum_crc32(b""), 4294967295);
        assert_eq!(cksum_crc32(b"A"), 1751207896);
        assert_eq!(cksum_crc32(b"#!/bin/sh\necho hello\n"), 1294090613);
    }

    #[test]
    fn header_magic_and_version() {
        let mut b = BomBuilder::new();
        b.add_file("/usr/local/bin/foo", 0o755, 0, 0, 0, b"hello");
        let bytes = build(&b);
        assert_eq!(&bytes[0..8], b"BOMStore");
        assert_eq!(&bytes[8..12], &1u32.to_be_bytes());
    }

    #[test]
    fn index_offset_plus_length_is_file_len() {
        let mut b = BomBuilder::new();
        b.add_file("/usr/local/bin/foo", 0o755, 0, 0, 0, b"hello");
        let bytes = build(&b);
        let index_offset = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
        let index_length = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
        assert_eq!((index_offset + index_length) as usize, bytes.len());
    }
}
