use crate::LivraisonResult;

pub fn gzip(content: &[u8]) -> LivraisonResult<Vec<u8>> {
    let mut out = Vec::new();
    zopfli::compress(
        zopfli::Options::default(),
        zopfli::Format::Gzip,
        content,
        &mut out,
    )?;
    Ok(out)
}
