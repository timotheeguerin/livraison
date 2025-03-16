use std::io::Write;

use crate::LivraisonResult;

use super::{builder::ArchiveBuilder, control::Control, tar::EnhancedTarBuilder};

pub struct LocalFile {
    pub dest: String,
    pub local_path: String,
}
pub struct InMemoryFile {
    pub dest: String,
    pub content: String,
    pub stats: FileStats,
}

pub struct FileStats {
    pub mode: u32,
}

pub enum DataFile {
    LocalFile(LocalFile),
    InMemoryFile(InMemoryFile),
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
        dbg!("Control tar added");
        archive.add_data(&self.create_data_tar()?)?;
        dbg!("Data tar added");

        archive.finish()?;
        Ok(archive)
    }

    fn create_control_tar(&self) -> LivraisonResult<Vec<u8>> {
        let mut tar_ar = EnhancedTarBuilder::new(Vec::new());

        tar_ar.add_file_from_bytes("control", self.control.write().as_bytes())?;
        if let Some(conf_files) = &self.conf_files {
            let content = self.create_conf_files_content(conf_files);
            tar_ar.add_file_from_bytes("conffiles", content.as_bytes())?;
        }
        tar_ar.finish()?;
        Ok(tar_ar.into_inner().unwrap())
    }

    fn create_conf_files_content(&self, conf_files: &[DataFile]) -> String {
        conf_files
            .iter()
            .map(|file| match file {
                DataFile::LocalFile(local_file) => local_file.dest.clone(),
                DataFile::InMemoryFile(in_memory_file) => in_memory_file.dest.clone(),
            })
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }

    fn create_data_tar(&self) -> LivraisonResult<Vec<u8>> {
        let mut tar_ar = EnhancedTarBuilder::new(Vec::new());

        if let Some(conf_files) = &self.conf_files {
            for file in conf_files {
                match file {
                    DataFile::LocalFile(local_file) => {
                        tar_ar.add_local_file(&local_file.dest, &local_file.local_path)?;
                    }
                    DataFile::InMemoryFile(in_memory_file) => {
                        tar_ar.add_file_from_bytes_with_stats(
                            &in_memory_file.dest,
                            in_memory_file.content.as_bytes(),
                            &in_memory_file.stats,
                        )?;
                    }
                }
            }
        }

        tar_ar.finish()?;
        Ok(tar_ar.into_inner().unwrap())
    }
}
