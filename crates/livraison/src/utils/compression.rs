use std::io::Write;

use libflate::gzip;

use crate::LivraisonResult;

pub fn gzip(content: &[u8]) -> LivraisonResult<Vec<u8>> {
    let mut encoder = gzip::Encoder::new(Vec::new())?;
    encoder.write_all(content)?;
    Ok(encoder.finish().into_result()?)
}
