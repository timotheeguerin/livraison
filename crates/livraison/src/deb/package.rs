use std::io::Write;

use crate::{LivraisonResult, common::FileRef};

use super::{builder::ArchiveBuilder, control::Control, tar::EnhancedTarBuilder};

#[derive(Debug)]
pub struct DataFile {
    dest: String,
    source: FileRef,
}

impl DataFile {
    pub fn new(dest: impl Into<String>, source: FileRef) -> Self {
        DataFile {
            dest: dest.into(),
            source,
        }
    }

    pub fn get_dest(&self) -> &str {
        &self.dest
    }

    pub fn get_source(&self) -> &FileRef {
        &self.source
    }
}

pub struct DebPackage {
    pub control: Control,
    pub files: Option<Vec<DataFile>>,
    pub conf_files: Option<Vec<DataFile>>,
}

impl DebPackage {
    pub fn write<W: Write>(&self, out: W) -> LivraisonResult<ArchiveBuilder<W>> {
        let mut archive = ArchiveBuilder::new(out)?;
        archive.add_control(&self.create_control_tar()?)?;
        let a = self.create_data_tar()?;
        archive.add_data(&a)?;
        archive.finish()?;
        Ok(archive)
    }

    fn create_control_tar(&self) -> LivraisonResult<Vec<u8>> {
        let mut tar_ar = EnhancedTarBuilder::new(Vec::new());

        tar_ar.add_file_from_text("control", self.control.write())?;
        if let Some(conf_files) = &self.conf_files {
            let content = self.create_conf_files_content(conf_files);
            tar_ar.add_file_from_text("conffiles", content)?;
        }
        tar_ar.finish()?;
        Ok(tar_ar.into_inner().unwrap())
    }

    fn create_conf_files_content(&self, conf_files: &[DataFile]) -> String {
        conf_files
            .iter()
            .map(|file| file.get_dest().to_string())
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }

    fn create_data_tar(&self) -> LivraisonResult<Vec<u8>> {
        let mut tar_ar = EnhancedTarBuilder::new(Vec::new());

        self.add_files_to_tar(&mut tar_ar, &self.files)?;
        self.add_files_to_tar(&mut tar_ar, &self.conf_files)?;

        tar_ar.finish()?;
        Ok(tar_ar.into_inner().unwrap())
    }

    fn add_files_to_tar(
        &self,
        tar_ar: &mut EnhancedTarBuilder<Vec<u8>>,
        files: &Option<Vec<DataFile>>,
    ) -> LivraisonResult<()> {
        if let Some(some_files) = files {
            for file in some_files {
                let _ = tar_ar.add_file(file.get_dest(), file.get_source());
            }
        }

        Ok(())
    }
}
