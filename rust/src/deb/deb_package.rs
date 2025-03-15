use std::io::Write;

use crate::LivraisonResult;

use super::{builder::ArchiveBuilder, control::Control};

pub struct DebPackage {
    pub control: Control,
}

impl DebPackage {
    pub fn write<W: Write>(&self, out: W) -> LivraisonResult<ArchiveBuilder<W>> {
        let mut archive = ArchiveBuilder::new(out)?;
        archive.add_control(&self.create_control_tar()?)?;
        archive.finish()?;
        Ok(archive)
    }

    fn create_control_tar(&self) -> LivraisonResult<Vec<u8>> {
        let mut tar_ar = tar::Builder::new(Vec::new());

        add_file_to_tar(
            &mut tar_ar,
            "control",
            self.control.write().as_bytes(),
            0o644,
        )?;
        tar_ar.finish()?;
        Ok(tar_ar.into_inner().unwrap())
    }
}

fn add_file_to_tar(
    tar_ar: &mut tar::Builder<Vec<u8>>,
    path: &str,
    out_data: &[u8],
    chmod: u32,
) -> LivraisonResult<()> {
    let mut header = tar::Header::new_gnu();
    header.set_mode(chmod);
    header.set_size(out_data.len() as u64);
    header.set_cksum();
    tar_ar.append_data(&mut header, path, out_data)?;
    Ok(())
}
