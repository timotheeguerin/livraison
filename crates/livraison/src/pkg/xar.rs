//! A minimal writer for the `xar` (eXtensible ARchive) container used by macOS
//! flat installer packages.
//!
//! Layout produced:
//! ```text
//! [ 28-byte header ][ zlib-compressed XML TOC ][ heap ]
//! ```
//! The heap begins with the SHA-1 digest of the compressed TOC (referenced by
//! the `<checksum>` element), followed by each file's data. Files are stored
//! uncompressed (`application/octet-stream`); any compression of the payload
//! itself (gzip cpio) is opaque to xar.

use sha1::{Digest, Sha1};

use crate::{LivraisonResult, utils::compression::zlib};

const MAGIC: u32 = 0x7861_7221; // "xar!"
const HEADER_SIZE: u16 = 28;
const VERSION: u16 = 1;
const CKSUM_SHA1: u32 = 1;
const CHECKSUM_LEN: u64 = 20;

/// A named entry to place in the archive heap.
pub struct XarFile {
    pub name: String,
    pub data: Vec<u8>,
}

fn sha1_hex(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Assemble the given files into a xar archive.
pub fn build(files: &[XarFile]) -> LivraisonResult<Vec<u8>> {
    // Heap positions start after the TOC checksum digest.
    let mut heap = Vec::new();
    let mut toc = String::new();
    toc.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    toc.push_str("<xar>\n <toc>\n");
    toc.push_str("  <checksum style=\"sha1\">\n");
    toc.push_str(&format!("   <size>{CHECKSUM_LEN}</size>\n"));
    toc.push_str("   <offset>0</offset>\n");
    toc.push_str("  </checksum>\n");

    for (i, file) in files.iter().enumerate() {
        let offset = CHECKSUM_LEN + heap.len() as u64;
        let len = file.data.len() as u64;
        let digest = sha1_hex(&file.data);
        heap.extend_from_slice(&file.data);

        toc.push_str(&format!("  <file id=\"{}\">\n", i + 1));
        toc.push_str(&format!("   <name>{}</name>\n", file.name));
        toc.push_str("   <type>file</type>\n");
        toc.push_str("   <data>\n");
        toc.push_str(&format!(
            "    <archived-checksum style=\"sha1\">{digest}</archived-checksum>\n"
        ));
        toc.push_str(&format!(
            "    <extracted-checksum style=\"sha1\">{digest}</extracted-checksum>\n"
        ));
        toc.push_str("    <encoding style=\"application/octet-stream\"/>\n");
        toc.push_str(&format!("    <size>{len}</size>\n"));
        toc.push_str(&format!("    <offset>{offset}</offset>\n"));
        toc.push_str(&format!("    <length>{len}</length>\n"));
        toc.push_str("   </data>\n");
        toc.push_str("  </file>\n");
    }
    toc.push_str(" </toc>\n</xar>\n");

    let toc_uncompressed = toc.into_bytes();
    let toc_compressed = zlib(&toc_uncompressed)?;

    // The heap's leading checksum is the SHA-1 of the compressed TOC.
    let mut toc_hasher = Sha1::new();
    toc_hasher.update(&toc_compressed);
    let toc_digest = toc_hasher.finalize();

    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC.to_be_bytes());
    out.extend_from_slice(&HEADER_SIZE.to_be_bytes());
    out.extend_from_slice(&VERSION.to_be_bytes());
    out.extend_from_slice(&(toc_compressed.len() as u64).to_be_bytes());
    out.extend_from_slice(&(toc_uncompressed.len() as u64).to_be_bytes());
    out.extend_from_slice(&CKSUM_SHA1.to_be_bytes());

    out.extend_from_slice(&toc_compressed);
    out.extend_from_slice(&toc_digest);
    out.extend_from_slice(&heap);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_has_magic_and_sha1_alg() {
        let out = build(&[XarFile {
            name: "PackageInfo".to_string(),
            data: b"<pkg-info/>".to_vec(),
        }])
        .unwrap();
        assert_eq!(&out[0..4], &MAGIC.to_be_bytes());
        assert_eq!(
            u16::from_be_bytes(out[4..6].try_into().unwrap()),
            HEADER_SIZE
        );
        assert_eq!(
            u32::from_be_bytes(out[24..28].try_into().unwrap()),
            CKSUM_SHA1
        );
    }
}
