use std::io::Write;

use crate::{LivraisonResult, common::FileStats};

use super::{builder::ArchiveBuilder, control::Control, tar::EnhancedTarBuilder};

#[derive(Debug, Clone)]
pub struct LocalFile {
    pub dest: String,
    pub local_path: String,
}
#[derive(Debug, Clone)]
pub struct InMemoryFile {
    pub dest: String,
    pub content: String,
    pub stats: FileStats,
}

#[derive(Debug, Clone)]
pub enum DataFile {
    LocalFile(LocalFile),
    InMemoryFile(InMemoryFile),
}

impl DataFile {
    pub fn get_dest(&self) -> &str {
        match self {
            DataFile::LocalFile(local_file) => &local_file.dest,
            DataFile::InMemoryFile(in_memory_file) => &in_memory_file.dest,
        }
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
        dbg!("DFAF");
        archive.add_data(&a)?;
        dbg!("Waiting finish.ar");
        archive.finish()?;
        dbg!("Done finish.ar");
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
            .map(|file| file.get_dest().to_string())
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }

    fn create_data_tar(&self) -> LivraisonResult<Vec<u8>> {
        let mut tar_ar = EnhancedTarBuilder::new(Vec::new());

        self.add_files_to_tar(&mut tar_ar, &self.files)?;
        self.add_files_to_tar(&mut tar_ar, &self.conf_files)?;

        dbg!("Waiting finish");
        tar_ar.finish()?;
        dbg!("Done finish");
        Ok(tar_ar.into_inner().unwrap())
    }

    fn add_files_to_tar(
        &self,
        tar_ar: &mut EnhancedTarBuilder<Vec<u8>>,
        files: &Option<Vec<DataFile>>,
    ) -> LivraisonResult<()> {
        if let Some(file) = &files {
            dbg!(file);
            for file in file {
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

        Ok(())
    }
}
