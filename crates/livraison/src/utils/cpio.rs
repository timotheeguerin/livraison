//! A minimal writer for the cpio archives used as package payloads.
//!
//! Two formats are supported:
//! - [`CpioFormat::Newc`] (SVR4, no CRC, magic `070701`): a fixed 110-byte
//!   ASCII-hex header, name and data each padded to a 4-byte boundary. Used as
//!   the RPM payload.
//! - [`CpioFormat::Odc`] (POSIX "old character", magic `070707`): a 76-byte
//!   ASCII-octal header with no padding. Used as the macOS `.pkg` payload, which
//!   the stricter `pkgutil`/libarchive reader expects.
//!
//! Every archive ends with a special `TRAILER!!!` entry.

use std::io::Read;

use crate::{LivraisonResult, common::FileRef};

const MAGIC_NEWC: &[u8] = b"070701";
const MAGIC_ODC: &[u8] = b"070707";

/// The on-disk cpio header format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpioFormat {
    /// SVR4 "new character" format (`070701`).
    Newc,
    /// POSIX "old character" format (`070707`).
    Odc,
}

pub struct CpioBuilder {
    out: Vec<u8>,
    /// Monotonic inode counter; values only need to be unique within the archive.
    ino: u32,
    format: CpioFormat,
}

impl Default for CpioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CpioBuilder {
    /// Create a builder producing the `newc` format (used by RPM).
    pub fn new() -> Self {
        Self::with_format(CpioFormat::Newc)
    }

    /// Create a builder producing the `odc` format (used by macOS `.pkg`).
    pub fn new_odc() -> Self {
        Self::with_format(CpioFormat::Odc)
    }

    pub fn with_format(format: CpioFormat) -> Self {
        CpioBuilder {
            out: Vec::new(),
            ino: 1,
            format,
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

    /// Add a directory entry (no data), e.g. `.` or `./usr/local/bin`.
    pub fn add_dir(&mut self, path: &str, mode: u32, mtime: u32) {
        self.write_entry(path, mode, mtime, 2, &[]);
        self.ino += 1;
    }

    /// Add a regular file from already-read bytes.
    pub fn add_file_bytes(&mut self, path: &str, mode: u32, mtime: u32, data: &[u8]) {
        self.write_entry(path, mode, mtime, 1, data);
        self.ino += 1;
    }

    /// Finish the archive, appending the trailer, and return the bytes.
    pub fn finish(mut self) -> Vec<u8> {
        self.write_entry("TRAILER!!!", 0, 0, 1, &[]);
        self.out
    }

    fn write_entry(&mut self, name: &str, mode: u32, mtime: u32, nlink: u32, data: &[u8]) {
        match self.format {
            CpioFormat::Newc => self.write_entry_newc(name, mode, mtime, nlink, data),
            CpioFormat::Odc => self.write_entry_odc(name, mode, mtime, nlink, data),
        }
    }

    fn write_entry_newc(&mut self, name: &str, mode: u32, mtime: u32, nlink: u32, data: &[u8]) {
        let name_bytes = name.as_bytes();
        let namesize = name_bytes.len() as u32 + 1; // include trailing NUL

        self.out.extend_from_slice(MAGIC_NEWC);
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

    fn write_entry_odc(&mut self, name: &str, mode: u32, mtime: u32, nlink: u32, data: &[u8]) {
        let name_bytes = name.as_bytes();
        let namesize = name_bytes.len() as u32 + 1; // include trailing NUL

        self.out.extend_from_slice(MAGIC_ODC);
        self.write_oct(0, 6); // dev
        self.write_oct(self.ino, 6);
        self.write_oct(mode, 6);
        self.write_oct(0, 6); // uid
        self.write_oct(0, 6); // gid
        self.write_oct(nlink, 6);
        self.write_oct(0, 6); // rdev
        self.write_oct(mtime, 11);
        self.write_oct(namesize, 6);
        self.write_oct(data.len() as u32, 11);

        // odc has no padding: name (with NUL) then data follow immediately.
        self.out.extend_from_slice(name_bytes);
        self.out.push(0);
        self.out.extend_from_slice(data);
    }

    fn write_hex(&mut self, value: u32) {
        // newc fields are 8 ASCII hex digits, zero padded, uppercase.
        let s = format!("{value:08X}");
        self.out.extend_from_slice(s.as_bytes());
    }

    fn write_oct(&mut self, value: u32, width: usize) {
        // odc fields are zero-padded ASCII octal of a fixed width.
        let s = format!("{value:0width$o}");
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
        assert_eq!(&bytes[0..6], MAGIC_NEWC);
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

    #[test]
    fn odc_header_uses_070707_and_no_padding() {
        let mut builder = CpioBuilder::new_odc();
        builder.add_dir(".", 0o40755, 0);
        builder
            .add_file(
                "./usr/local/bin/foo",
                0o100755,
                0,
                &FileRef::from_text("hi"),
            )
            .unwrap();
        let bytes = builder.finish();
        assert_eq!(&bytes[0..6], MAGIC_ODC);
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("./usr/local/bin/foo"));
        assert!(text.contains("TRAILER!!!"));
    }
}
