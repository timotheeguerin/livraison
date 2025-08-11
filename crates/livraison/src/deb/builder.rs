use crate::LivraisonResult;
use crate::utils::compression::gzip;
use ar::{Builder, Header};
use std::io::Write;

pub struct ArchiveBuilder<W: Write> {
    ar_builder: Builder<W>,
}

impl<W: Write> ArchiveBuilder<W> {
    pub fn new(writer: W) -> LivraisonResult<ArchiveBuilder<W>> {
        let ar_builder = Builder::new(writer);

        let mut ar = ArchiveBuilder { ar_builder };
        ar.add_file("debian-binary", b"2.0\n")?;
        Ok(ar)
    }

    pub fn add_control(&mut self, tar: &[u8]) -> LivraisonResult<()> {
        self.add_file("control.tar.gz", &gzip(tar)?)
    }

    pub fn add_data(&mut self, tar: &[u8]) -> LivraisonResult<()> {
        dbg!("Before Compressed data size: {}", tar.len());
        let comp = gzip(tar)?;
        dbg!("Compressed data size: {}", comp.len());
        self.add_file("data.tar.gz", &comp)
    }

    fn add_file(&mut self, dest_path: &str, data: &[u8]) -> LivraisonResult<()> {
        let mut header = Header::new(dest_path.into(), data.len() as u64);
        header.set_mode(0o100644); // dpkg uses 100644
        header.set_uid(0);
        header.set_gid(0);
        self.ar_builder.append(&header, data)?;
        Ok(())
    }

    pub fn finish(&self) -> LivraisonResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
