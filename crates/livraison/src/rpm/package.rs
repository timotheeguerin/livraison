//! Assembly of a complete `.rpm` file: lead + signature header + main header +
//! gzip-compressed cpio payload.

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use md5::Md5;
use sha2::{Digest, Sha256};

use crate::{LivraisonResult, common::FileRef, utils::compression::gzip};

use super::{
    constants::*,
    cpio::CpioBuilder,
    header::{Entry, TypedData, write_header},
    lead::write_lead,
    metadata::RpmMetadata,
};

/// A file to include in the package, installed at `dest` (an absolute path).
#[derive(Debug, Clone)]
pub struct DataFile {
    dest: String,
    source: FileRef,
}

impl DataFile {
    pub fn new(dest: impl Into<String>, source: FileRef) -> Self {
        DataFile {
            dest: dest.into(),
            source,
        }
    }

    pub fn get_dest(&self) -> &str {
        &self.dest
    }
}

/// A full RPM package ready to be serialized.
pub struct RpmPackage {
    pub metadata: RpmMetadata,
    pub files: Vec<DataFile>,
}

/// Split an absolute path into `(dirname, basename)` where `dirname` keeps its
/// trailing slash, as required by RPM's `DIRNAMES`/`BASENAMES` tags.
fn split_path(path: &str) -> (String, String) {
    match path.rfind('/') {
        Some(idx) => (path[..=idx].to_string(), path[idx + 1..].to_string()),
        None => ("/".to_string(), path.to_string()),
    }
}

/// Ensure a mode has the regular-file type bits set.
fn as_file_mode(mode: u32) -> u32 {
    if mode & 0o170000 == 0 {
        mode | 0o100000
    } else {
        mode
    }
}

impl RpmPackage {
    pub fn write<W: Write>(&self, mut out: W) -> LivraisonResult<()> {
        let mtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);

        // --- Gather per-file metadata and build the cpio payload. ---
        let mut basenames = Vec::new();
        let mut dirnames: Vec<String> = Vec::new();
        let mut dirindexes = Vec::new();
        let mut filesizes = Vec::new();
        let mut filemodes = Vec::new();
        let mut filemtimes = Vec::new();
        let mut filedigests = Vec::new();
        let mut fileflags = Vec::new();
        let mut fileusername = Vec::new();
        let mut filegroupname = Vec::new();
        let mut total_size: u32 = 0;

        let mut cpio = CpioBuilder::new();

        for file in &self.files {
            let (dirname, basename) = split_path(&file.dest);
            let dir_index = match dirnames.iter().position(|d| d == &dirname) {
                Some(i) => i as u32,
                None => {
                    dirnames.push(dirname.clone());
                    (dirnames.len() - 1) as u32
                }
            };

            let mut data = Vec::new();
            std::io::Read::read_to_end(&mut file.source.open()?, &mut data)?;
            let size = data.len() as u32;
            total_size += size;

            let mut hasher = Sha256::new();
            hasher.update(&data);
            let digest = hex(&hasher.finalize());

            let mode = as_file_mode(file.source.get_mode().unwrap_or(0o100755));

            cpio.add_file(&format!(".{}", file.dest), mode, mtime, &file.source)?;

            basenames.push(basename);
            dirindexes.push(dir_index);
            filesizes.push(size);
            filemodes.push(mode as u16);
            filemtimes.push(mtime);
            filedigests.push(digest);
            fileflags.push(0u32);
            fileusername.push("root".to_string());
            filegroupname.push("root".to_string());
        }

        let payload = gzip(&cpio.finish())?;

        // --- Build the main (immutable) header. ---
        let mut records = vec![
            Entry::new(RPMTAG_NAME, TypedData::Str(self.metadata.name.clone())),
            Entry::new(RPMTAG_VERSION, TypedData::Str(self.metadata.version.clone())),
            Entry::new(RPMTAG_RELEASE, TypedData::Str(self.metadata.release.clone())),
            Entry::new(RPMTAG_SUMMARY, TypedData::Str(self.metadata.summary.clone())),
            Entry::new(
                RPMTAG_DESCRIPTION,
                TypedData::Str(self.metadata.description.clone()),
            ),
            Entry::new(RPMTAG_LICENSE, TypedData::Str(self.metadata.license.clone())),
            Entry::new(RPMTAG_OS, TypedData::Str("linux".to_string())),
            Entry::new(RPMTAG_ARCH, TypedData::Str(self.metadata.arch.clone())),
            Entry::new(RPMTAG_SIZE, TypedData::Int32(vec![total_size])),
            Entry::new(RPMTAG_RPMVERSION, TypedData::Str("4.0".to_string())),
            Entry::new(RPMTAG_PAYLOADFORMAT, TypedData::Str("cpio".to_string())),
            Entry::new(RPMTAG_PAYLOADCOMPRESSOR, TypedData::Str("gzip".to_string())),
            Entry::new(RPMTAG_PAYLOADFLAGS, TypedData::Str("9".to_string())),
            Entry::new(RPMTAG_ENCODING, TypedData::Str("utf-8".to_string())),
        ];

        if !self.files.is_empty() {
            records.push(Entry::new(RPMTAG_BASENAMES, TypedData::StringArray(basenames)));
            records.push(Entry::new(RPMTAG_DIRNAMES, TypedData::StringArray(dirnames)));
            records.push(Entry::new(RPMTAG_DIRINDEXES, TypedData::Int32(dirindexes)));
            records.push(Entry::new(RPMTAG_FILESIZES, TypedData::Int32(filesizes)));
            records.push(Entry::new(RPMTAG_FILEMODES, TypedData::Int16(filemodes)));
            records.push(Entry::new(RPMTAG_FILEMTIMES, TypedData::Int32(filemtimes)));
            records.push(Entry::new(
                RPMTAG_FILEDIGESTS,
                TypedData::StringArray(filedigests),
            ));
            records.push(Entry::new(RPMTAG_FILEFLAGS, TypedData::Int32(fileflags)));
            records.push(Entry::new(
                RPMTAG_FILEUSERNAME,
                TypedData::StringArray(fileusername),
            ));
            records.push(Entry::new(
                RPMTAG_FILEGROUPNAME,
                TypedData::StringArray(filegroupname),
            ));
            records.push(Entry::new(
                RPMTAG_FILEDIGESTALGO,
                TypedData::Int32(vec![RPM_DIGEST_ALGO_SHA256]),
            ));
        }

        let mut header_bytes = Vec::new();
        write_header(&mut header_bytes, records, RPMTAG_HEADERIMMUTABLE)?;

        // --- Build the signature header over the main header + payload. ---
        let mut sha = Sha256::new();
        sha.update(&header_bytes);
        let header_sha256 = hex(&sha.finalize());

        let mut md5 = Md5::new();
        md5.update(&header_bytes);
        md5.update(&payload);
        let md5_digest = md5.finalize().to_vec();

        let sig_size = (header_bytes.len() + payload.len()) as u32;
        let sig_records = vec![
            Entry::new(RPMSIGTAG_SIZE, TypedData::Int32(vec![sig_size])),
            Entry::new(RPMSIGTAG_SHA256, TypedData::Str(header_sha256)),
            Entry::new(RPMSIGTAG_MD5, TypedData::Bin(md5_digest)),
        ];
        let mut sig_bytes = Vec::new();
        write_header(&mut sig_bytes, sig_records, RPMTAG_HEADERSIGNATURES)?;
        // The signature header is padded to an 8-byte boundary.
        while !sig_bytes.len().is_multiple_of(8) {
            sig_bytes.push(0);
        }

        // --- Emit the file. ---
        write_lead(&mut out, &self.metadata.name)?;
        out.write_all(&sig_bytes)?;
        out.write_all(&header_bytes)?;
        out.write_all(&payload)?;
        Ok(())
    }
}

/// Lowercase hex encoding.
fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_path_keeps_trailing_slash() {
        assert_eq!(
            split_path("/usr/local/bin/foo"),
            ("/usr/local/bin/".to_string(), "foo".to_string())
        );
    }

    #[test]
    fn as_file_mode_adds_regular_file_bits() {
        assert_eq!(as_file_mode(0o755), 0o100755);
        assert_eq!(as_file_mode(0o100644), 0o100644);
    }

    #[test]
    fn hex_encodes_lowercase() {
        assert_eq!(hex(&[0x00, 0xab, 0xff]), "00abff");
    }

    #[test]
    fn writes_lead_magic() {
        let pkg = RpmPackage {
            metadata: RpmMetadata {
                name: "test".to_string(),
                ..Default::default()
            },
            files: vec![DataFile::new(
                "/usr/local/bin/test",
                FileRef::from_text("#!/bin/sh\n"),
            )],
        };
        let mut out = Vec::new();
        pkg.write(&mut out).unwrap();
        assert_eq!(&out[0..4], &RPM_MAGIC);
    }
}
