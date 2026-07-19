//! A minimal writer for the `newc` (SVR4, no CRC) cpio format used as the RPM
//! payload.
//!
//! Each entry is a fixed 110-byte ASCII-hex header, followed by the NUL
//! terminated name and the file data, each padded to a 4-byte boundary. The
//! archive ends with a special `TRAILER!!!` entry.

use std::io::Read;

use crate::{LivraisonResult, common::FileRef};

const MAGIC: &[u8] = b"070701";

pub struct CpioBuilder {
    out: Vec<u8>,
    /// Monotonic inode counter; values only need to be unique within the archive.
    ino: u32,
}

impl Default for CpioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CpioBuilder {
    pub fn new() -> Self {
        CpioBuilder {
            out: Vec::new(),
            ino: 1,
        }
    }

    /// Add a regular file. `path` is the archive path, conventionally prefixed
    /// with `./` (e.g. `./usr/local/bin/foo`).
    pub fn add_file(
        &mut self,
        path: &str,
        mode: u32,
        mtime: u32,
        source: &FileRef,
    ) -> LivraisonResult<()> {
        let mut data = Vec::new();
        source.open()?.read_to_end(&mut data)?;
        self.write_entry(path, mode, mtime, 1, &data);
        self.ino += 1;
        Ok(())
    }

    /// Finish the archive, appending the trailer, and return the bytes.
    pub fn finish(mut self) -> Vec<u8> {
        self.write_entry("TRAILER!!!", 0, 0, 1, &[]);
        self.out
    }

    fn write_entry(&mut self, name: &str, mode: u32, mtime: u32, nlink: u32, data: &[u8]) {
        let name_bytes = name.as_bytes();
        let namesize = name_bytes.len() as u32 + 1; // include trailing NUL

        self.out.extend_from_slice(MAGIC);
        self.write_hex(self.ino);
        self.write_hex(mode);
        self.write_hex(0); // uid
        self.write_hex(0); // gid
        self.write_hex(nlink);
        self.write_hex(mtime);
        self.write_hex(data.len() as u32);
        self.write_hex(0); // devmajor
        self.write_hex(0); // devminor
        self.write_hex(0); // rdevmajor
        self.write_hex(0); // rdevminor
        self.write_hex(namesize);
        self.write_hex(0); // check

        self.out.extend_from_slice(name_bytes);
        self.out.push(0);
        // The name field (starting after the 110-byte header) is padded so the
        // file data begins on a 4-byte boundary.
        self.pad_to_4();

        self.out.extend_from_slice(data);
        self.pad_to_4();
    }

    fn write_hex(&mut self, value: u32) {
        // newc fields are 8 ASCII hex digits, zero padded, uppercase.
        let s = format!("{value:08X}");
        self.out.extend_from_slice(s.as_bytes());
    }

    fn pad_to_4(&mut self) {
        while !self.out.len().is_multiple_of(4) {
            self.out.push(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_starts_with_magic() {
        let builder = CpioBuilder::new();
        let bytes = builder.finish();
        assert_eq!(&bytes[0..6], MAGIC);
    }

    #[test]
    fn trailer_is_present() {
        let bytes = CpioBuilder::new().finish();
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("TRAILER!!!"));
    }

    #[test]
    fn file_entry_is_4_byte_aligned() {
        let mut builder = CpioBuilder::new();
        builder
            .add_file(
                "./usr/local/bin/foo",
                0o100755,
                0,
                &FileRef::from_text("hello world"),
            )
            .unwrap();
        let bytes = builder.finish();
        assert!(bytes.len().is_multiple_of(4));
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("./usr/local/bin/foo"));
        assert!(text.contains("hello world"));
    }
}
