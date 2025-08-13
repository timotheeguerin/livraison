use std::io::Write;

use flate2::{Compression, write::GzEncoder};

use crate::LivraisonResult;

pub fn gzip(content: &[u8]) -> LivraisonResult<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    Ok(encoder.finish()?)
}
