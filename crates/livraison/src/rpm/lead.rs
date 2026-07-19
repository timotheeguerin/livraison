//! The 96-byte RPM *lead* section.
//!
//! The lead is a legacy structure. Modern RPM tooling only uses its magic bytes
//! to recognise a file as an RPM; the remaining fields are written with the
//! conventional fixed values.

use std::io::Write;

use crate::LivraisonResult;

use super::constants::RPM_MAGIC;

/// Write the 96-byte lead for a binary package named `name`.
pub fn write_lead<W: Write>(out: &mut W, name: &str) -> LivraisonResult<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(96);
    buf.extend_from_slice(&RPM_MAGIC);
    buf.push(3); // major version
    buf.push(0); // minor version
    buf.extend_from_slice(&0u16.to_be_bytes()); // type: 0 = binary
    buf.extend_from_slice(&0u16.to_be_bytes()); // archnum

    // name, 66 bytes, NUL padded, always NUL terminated.
    let mut name_field = [0u8; 66];
    let bytes = name.as_bytes();
    let len = bytes.len().min(65);
    name_field[..len].copy_from_slice(&bytes[..len]);
    buf.extend_from_slice(&name_field);

    buf.extend_from_slice(&1u16.to_be_bytes()); // osnum: 1 = Linux
    buf.extend_from_slice(&5u16.to_be_bytes()); // signature type: 5 = header-style
    buf.extend_from_slice(&[0u8; 16]); // reserved

    out.write_all(&buf)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lead_is_96_bytes_with_magic() {
        let mut out = Vec::new();
        write_lead(&mut out, "test").unwrap();
        assert_eq!(out.len(), 96);
        assert_eq!(&out[0..4], &RPM_MAGIC);
        assert_eq!(out[4], 3);
    }

    #[test]
    fn name_is_written_and_nul_terminated() {
        let mut out = Vec::new();
        write_lead(&mut out, "hello").unwrap();
        assert_eq!(&out[10..15], b"hello");
        assert_eq!(out[15], 0);
    }

    #[test]
    fn overlong_name_is_truncated_and_terminated() {
        let long = "a".repeat(100);
        let mut out = Vec::new();
        write_lead(&mut out, &long).unwrap();
        // 66-byte field, last byte must remain the NUL terminator.
        assert_eq!(out[10 + 65], 0);
    }
}
