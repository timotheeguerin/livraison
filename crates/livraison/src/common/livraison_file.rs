use std::{fs, os::unix::fs::MetadataExt};

pub enum LivraisonFile {
    Local(fs::File),
    InMemory {
        name: String,
        content: String,
        stats: super::FileStats,
    },
}

impl LivraisonFile {
    pub fn from_local(file: fs::File) -> Self {
        LivraisonFile::Local(file)
    }

    pub fn metadata(&'_ self) -> LivraisonFileMetadata<'_> {
        LivraisonFileMetadata(self)
    }
}

pub struct LivraisonFileMetadata<'a>(&'a LivraisonFile);

impl<'a> LivraisonFileMetadata<'a> {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u64 {
        match &self.0 {
            LivraisonFile::Local(file) => file.metadata().unwrap().len(),
            LivraisonFile::InMemory { content, .. } => content.len() as u64,
        }
    }

    pub fn mode(&self) -> u32 {
        match &self.0 {
            LivraisonFile::Local(file) => file.metadata().unwrap().mode(),
            LivraisonFile::InMemory { stats, .. } => stats.mode,
        }
    }
}
