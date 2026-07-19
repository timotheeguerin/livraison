//! Serialization of an RPM *header* structure.
//!
//! A header is a self-describing collection of tagged records. It is used both
//! for the signature section and for the main (immutable) metadata section of an
//! RPM package. On disk a header is laid out as:
//!
//! ```text
//! magic (3) | version (1) | reserved (4) | nentries (4) | store_size (4)
//! nentries * index-entry (16 bytes each: tag, type, offset, count)
//! data store (store_size bytes)
//! ```
//!
//! Only serialization is implemented; livraison never needs to parse RPMs.

use std::io::Write;

use crate::LivraisonResult;

use super::constants::{HEADER_MAGIC, INDEX_ENTRY_SIZE};

/// The value/type pair stored by a single [`Entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedData {
    Int16(Vec<u16>),
    Int32(Vec<u32>),
    Str(String),
    StringArray(Vec<String>),
    Bin(Vec<u8>),
}

impl TypedData {
    /// The RPM type identifier used in the index entry.
    fn type_id(&self) -> u32 {
        match self {
            TypedData::Int16(_) => 3,
            TypedData::Int32(_) => 4,
            TypedData::Str(_) => 6,
            TypedData::Bin(_) => 7,
            TypedData::StringArray(_) => 8,
        }
    }

    /// The number of items pointed to by the index entry.
    fn count(&self) -> u32 {
        match self {
            TypedData::Int16(v) => v.len() as u32,
            TypedData::Int32(v) => v.len() as u32,
            TypedData::Str(_) => 1,
            TypedData::Bin(v) => v.len() as u32,
            TypedData::StringArray(v) => v.len() as u32,
        }
    }

    /// Append the data to the store, inserting any leading padding required to
    /// satisfy this type's alignment. Returns the number of padding bytes added.
    fn append(&self, store: &mut Vec<u8>) -> u32 {
        match self {
            TypedData::Int16(v) => {
                let pad = align(store, 2);
                for item in v {
                    store.extend_from_slice(&item.to_be_bytes());
                }
                pad
            }
            TypedData::Int32(v) => {
                let pad = align(store, 4);
                for item in v {
                    store.extend_from_slice(&item.to_be_bytes());
                }
                pad
            }
            TypedData::Str(s) => {
                store.extend_from_slice(s.as_bytes());
                store.push(0);
                0
            }
            TypedData::Bin(b) => {
                store.extend_from_slice(b);
                0
            }
            TypedData::StringArray(v) => {
                for s in v {
                    store.extend_from_slice(s.as_bytes());
                    store.push(0);
                }
                0
            }
        }
    }
}

/// Pad `store` with zero bytes until its length is a multiple of `alignment`.
/// Returns the number of padding bytes added.
fn align(store: &mut Vec<u8>, alignment: usize) -> u32 {
    let mut pad = 0;
    while !store.len().is_multiple_of(alignment) {
        store.push(0);
        pad += 1;
    }
    pad
}

/// A single tagged record within a header.
#[derive(Debug, Clone)]
pub struct Entry {
    pub tag: u32,
    pub data: TypedData,
}

impl Entry {
    pub fn new(tag: u32, data: TypedData) -> Self {
        Entry { tag, data }
    }
}

/// Write a single 16-byte index entry (tag, type, offset, count).
fn write_index_entry(out: &mut Vec<u8>, tag: u32, type_id: u32, offset: i32, count: u32) {
    out.extend_from_slice(&tag.to_be_bytes());
    out.extend_from_slice(&type_id.to_be_bytes());
    out.extend_from_slice(&offset.to_be_bytes());
    out.extend_from_slice(&count.to_be_bytes());
}

/// Serialize a set of records into a full header, wrapped in the given region
/// tag (`RPMTAG_HEADERIMMUTABLE` for the main header, `RPMTAG_HEADERSIGNATURES`
/// for the signature header).
///
/// Entries are sorted by tag, the region trailer is computed, and the whole
/// structure is written to `out`. Returns the number of bytes written.
pub fn write_header<W: Write>(
    out: &mut W,
    mut records: Vec<Entry>,
    region_tag: u32,
) -> LivraisonResult<usize> {
    records.sort_by_key(|e| e.tag);

    // Build the data store and the index entries for the actual records.
    let mut store: Vec<u8> = Vec::new();
    let mut index: Vec<u8> = Vec::new();
    // The region tag is always the first index entry, so actual entries start
    // at offset INDEX_ENTRY_SIZE within the index section. Reserve its slot.
    let mut region_index: Vec<u8> = Vec::new();

    for record in &records {
        let offset_before = store.len() as i32;
        let pad = record.data.append(&mut store) as i32;
        let offset = offset_before + pad;
        write_index_entry(
            &mut index,
            record.tag,
            record.data.type_id(),
            offset,
            record.data.count(),
        );
    }

    // The region trailer is a 16-byte index entry stored in the data store. Its
    // offset points backwards over every index entry (region + actual records).
    let record_count = records.len() as i32;
    let trailer_offset = -((record_count + 1) * INDEX_ENTRY_SIZE);
    let mut trailer: Vec<u8> = Vec::new();
    write_index_entry(&mut trailer, region_tag, 7, trailer_offset, INDEX_ENTRY_SIZE as u32);

    // The region index entry itself points at the trailer we are about to append.
    let region_data_offset = store.len() as i32;
    write_index_entry(
        &mut region_index,
        region_tag,
        7,
        region_data_offset,
        INDEX_ENTRY_SIZE as u32,
    );
    store.extend_from_slice(&trailer);

    let num_entries = (records.len() + 1) as u32;
    let store_size = store.len() as u32;

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&HEADER_MAGIC);
    buf.push(1); // version
    buf.extend_from_slice(&[0; 4]); // reserved
    buf.extend_from_slice(&num_entries.to_be_bytes());
    buf.extend_from_slice(&store_size.to_be_bytes());
    buf.extend_from_slice(&region_index);
    buf.extend_from_slice(&index);
    buf.extend_from_slice(&store);

    out.write_all(&buf)?;
    Ok(buf.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpm::constants::{RPMTAG_HEADERIMMUTABLE, RPMTAG_NAME, RPMTAG_VERSION};

    fn parse_u32(bytes: &[u8], at: usize) -> u32 {
        u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap())
    }

    #[test]
    fn writes_magic_and_counts() {
        let mut out = Vec::new();
        write_header(
            &mut out,
            vec![
                Entry::new(RPMTAG_VERSION, TypedData::Str("1.0".to_string())),
                Entry::new(RPMTAG_NAME, TypedData::Str("test".to_string())),
            ],
            RPMTAG_HEADERIMMUTABLE,
        )
        .unwrap();

        assert_eq!(&out[0..3], &HEADER_MAGIC);
        assert_eq!(out[3], 1);
        // num_entries = 2 records + 1 region tag
        assert_eq!(parse_u32(&out, 8), 3);
    }

    #[test]
    fn sorts_entries_by_tag_after_region() {
        let mut out = Vec::new();
        write_header(
            &mut out,
            vec![
                Entry::new(RPMTAG_VERSION, TypedData::Str("1.0".to_string())),
                Entry::new(RPMTAG_NAME, TypedData::Str("test".to_string())),
            ],
            RPMTAG_HEADERIMMUTABLE,
        )
        .unwrap();

        // Index section starts at byte 16. First entry is the region tag.
        let first_tag = parse_u32(&out, 16);
        assert_eq!(first_tag, RPMTAG_HEADERIMMUTABLE);
        // Next two entries must be sorted: NAME (1000) before VERSION (1001).
        assert_eq!(parse_u32(&out, 16 + 16), RPMTAG_NAME);
        assert_eq!(parse_u32(&out, 16 + 32), RPMTAG_VERSION);
    }

    #[test]
    fn region_trailer_offset_points_back_over_all_entries() {
        let mut out = Vec::new();
        write_header(
            &mut out,
            vec![Entry::new(RPMTAG_NAME, TypedData::Str("test".to_string()))],
            RPMTAG_HEADERIMMUTABLE,
        )
        .unwrap();

        let num_entries = parse_u32(&out, 8);
        let store_size = parse_u32(&out, 12) as usize;
        let store_start = 16 + (num_entries as usize) * 16;
        let store = &out[store_start..store_start + store_size];
        // The trailer is the last 16 bytes of the store.
        let trailer = &store[store.len() - 16..];
        let offset = i32::from_be_bytes(trailer[8..12].try_into().unwrap());
        // 2 entries total (region + NAME) => -(2 * 16)
        assert_eq!(offset, -(2 * INDEX_ENTRY_SIZE));
    }

    #[test]
    fn int32_data_is_4_byte_aligned() {
        let mut out = Vec::new();
        // A short string (odd length incl. NUL) followed by an int32 forces padding.
        write_header(
            &mut out,
            vec![
                Entry::new(1000, TypedData::Str("ab".to_string())),
                Entry::new(1001, TypedData::Int32(vec![0x0a0b0c0d])),
            ],
            RPMTAG_HEADERIMMUTABLE,
        )
        .unwrap();

        let num_entries = parse_u32(&out, 8);
        let store_start = 16 + (num_entries as usize) * 16;
        // "ab\0" occupies 3 bytes; int32 must be padded to offset 4.
        // Find the int32 entry (tag 1001) and read its offset.
        let mut int_offset = None;
        for i in 0..num_entries as usize {
            let base = 16 + i * 16;
            if parse_u32(&out, base) == 1001 {
                int_offset = Some(i32::from_be_bytes(
                    out[base + 8..base + 12].try_into().unwrap(),
                ));
            }
        }
        let int_offset = int_offset.unwrap();
        assert_eq!(int_offset % 4, 0);
        let abs = store_start + int_offset as usize;
        assert_eq!(&out[abs..abs + 4], &[0x0a, 0x0b, 0x0c, 0x0d]);
    }
}
