use std::io::Write;

use flate2::{Compression, write::GzEncoder, write::ZlibEncoder};

use crate::LivraisonResult;

pub fn gzip(content: &[u8]) -> LivraisonResult<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    Ok(encoder.finish()?)
}

/// zlib (RFC 1950) compression, used for the xar table-of-contents.
pub fn zlib(content: &[u8]) -> LivraisonResult<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    Ok(encoder.finish()?)
}
