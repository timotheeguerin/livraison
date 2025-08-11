use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LocalFile {
    pub local_path: PathBuf,
    pub dest: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InMemoryFile {
    pub dest: Option<String>,
    pub name: String,
    pub content: String,
    pub stats: FileStats,
}

#[derive(Debug, Clone)]
pub struct FileStats {
    pub mode: u32,
}

#[derive(Debug, Clone)]
pub enum DataFile {
    LocalFile(LocalFile),
    InMemoryFile(InMemoryFile),
}

impl DataFile {
    pub fn from_local(s: impl Into<PathBuf>) -> DataFile {
        DataFile::LocalFile(LocalFile {
            local_path: s.into(),
            dest: None,
        })
    }

    pub fn file_name(&self) -> String {
        match self {
            DataFile::LocalFile(file) => file
                .local_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            DataFile::InMemoryFile(file) => file.name.clone(),
        }
    }
}
