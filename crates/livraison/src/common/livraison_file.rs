use std::fs;
#[cfg(all(unix, not(target_arch = "wasm32")))]
use std::os::unix::prelude::MetadataExt;

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
            LivraisonFile::Local(file) => get_mode_from_metadata(&file.metadata().unwrap()),
            LivraisonFile::InMemory { stats, .. } => stats.mode,
        }
    }
}

#[cfg(any(windows, target_arch = "wasm32"))]
#[allow(unused_variables)]
fn get_mode_from_metadata(meta: &fs::Metadata) -> u32 {
    0o644
}

#[cfg(all(unix, not(target_arch = "wasm32")))]
fn get_mode_from_metadata(file: &fs::Metadata) -> u32 {
    file.mode()
}
