use std::io::Write;

use libflate::gzip;

use crate::LivraisonResult;

pub fn gzip(content: &[u8]) -> LivraisonResult<Vec<u8>> {
    let mut result = Vec::new();
    let mut encoder = gzip::Encoder::new(&mut result)?;
    encoder.write_all(content)?;
    Ok(result)
}
