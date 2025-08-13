#[cfg(all(unix, not(target_arch = "wasm32")))]
use std::os::unix::prelude::MetadataExt;
use std::{
    fs::{self, File},
    io::{Cursor, Read},
    path::PathBuf,
};

/// Declarative reference to a file to be used as input for creating archives.
/// Features:
///  - Using a local file or some text as input.
///  - Set the permission mode that this file should apply
///
/// Example:
/// ```rust
/// use livraison::common::FileRef;
///
/// let file = FileRef::from_local("/path/to/file");
/// let binary = FileRef::from_local("/path/to/binary").with_mode(0o755);
/// ```
#[derive(Debug, Clone)]
pub struct FileRef {
    inner: FileContentSource,
    mode: Option<u32>,
}

pub struct OpenedFileRef {
    reader: OpenedFileReader,
}

enum OpenedFileReader {
    File(File),
    Memory(Cursor<Vec<u8>>),
}

impl Read for OpenedFileRef {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.reader {
            OpenedFileReader::File(file) => file.read(buf),
            OpenedFileReader::Memory(cursor) => cursor.read(buf),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileContentSource {
    Local(PathBuf),
    InMemory { path: PathBuf, content: String },
}

impl FileRef {
    pub fn from_local(path: impl Into<PathBuf>) -> Self {
        FileRef {
            inner: FileContentSource::Local(path.into()),
            mode: None,
        }
    }

    pub fn from_text(content: impl Into<String>) -> Self {
        FileRef {
            inner: FileContentSource::InMemory {
                path: PathBuf::from(""),
                content: content.into(),
            },
            mode: None,
        }
    }

    pub fn from_text_and_name(name: impl Into<PathBuf>, content: String) -> Self {
        FileRef {
            inner: FileContentSource::InMemory {
                path: name.into(),
                content,
            },
            mode: None,
        }
    }

    pub fn file_name(&self) -> String {
        self.inner.file_name()
    }

    pub fn open(&'_ self) -> std::io::Result<OpenedFileRef> {
        let reader = match &self.inner {
            FileContentSource::Local(file) => OpenedFileReader::File(File::open(file)?),
            FileContentSource::InMemory { content, .. } => {
                OpenedFileReader::Memory(Cursor::new(content.as_bytes().to_vec()))
            }
        };

        Ok(OpenedFileRef { reader })
    }

    /// Set the file mode
    pub fn with_mode(mut self, mode: u32) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Get the requested mode for this file
    pub fn get_mode(&self) -> Option<u32> {
        if self.mode.is_some() {
            self.mode
        } else {
            self.inner.mode()
        }
    }

    /// Get the length of the file
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u64 {
        self.inner.len()
    }
}
impl FileContentSource {
    pub fn file_name(&self) -> String {
        self.get_path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }

    fn get_path(&self) -> &PathBuf {
        match &self {
            FileContentSource::Local(path) => path,
            FileContentSource::InMemory { path, .. } => path,
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u64 {
        match &self {
            FileContentSource::Local(path) => path.metadata().unwrap().len(),
            FileContentSource::InMemory { content, .. } => content.len() as u64,
        }
    }

    pub fn mode(&self) -> Option<u32> {
        match &self {
            FileContentSource::Local(file) => {
                Some(get_mode_from_metadata(&file.metadata().unwrap()))
            }
            FileContentSource::InMemory { .. } => None,
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
