use std::{
    collections::HashSet,
    fs,
    io::{self, Write},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{LivraisonResult, common::FileStats};

pub struct EnhancedTarBuilder<W: Write> {
    builder: tar::Builder<W>,
    dirs_added: HashSet<PathBuf>,
    mtime: u64,
}

/// EnhancedTar is a wrapper around tar::Builder that automatically adds intermediate directories
/// when adding files to the archive and provide a few more apis.
impl<W: Write> EnhancedTarBuilder<W> {
    pub fn new(writer: W) -> Self {
        let builder = tar::Builder::new(writer);

        let mtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        EnhancedTarBuilder {
            builder,
            dirs_added: HashSet::new(),
            mtime,
        }
    }

    pub fn finish(&mut self) -> LivraisonResult<()> {
        self.builder.finish()?;
        Ok(())
    }

    pub fn into_inner(self) -> LivraisonResult<W> {
        Ok(self.builder.into_inner()?)
    }

    pub fn add_file_from_bytes(&mut self, dest_path: &str, data: &[u8]) -> LivraisonResult<()> {
        self.add_file_from_bytes_with_stats(dest_path, data, &FileStats { mode: 0o644 })?;
        Ok(())
    }
    pub fn add_file_from_bytes_with_stats(
        &mut self,
        dest_path: &str,
        data: &[u8],
        stats: &FileStats,
    ) -> LivraisonResult<()> {
        let dest_path = Path::new(dest_path.trim_start_matches('/'));
        self.add_parent_dirs(dest_path)?;

        let mut header = tar::Header::new_gnu();
        header.set_mtime(self.mtime);
        header.set_mode(stats.mode);
        header.set_size(data.len() as u64);
        header.set_cksum();
        self.builder.append_data(&mut header, dest_path, data)?;
        Ok(())
    }

    pub fn add_local_file(&mut self, dest_path: &str, local_path: &str) -> LivraisonResult<()> {
        let dest_path_p = Path::new(dest_path.trim_start_matches('/'));
        self.add_parent_dirs(dest_path_p)?;

        dbg!("HERE1");
        // let mut file = fs::File::open(local_path)?;
        // self.builder.append_file(dest_path, &mut file)?;
        let file = fs::read(local_path)?;
        dbg!("BYtes", file.len());
        self.add_file_from_bytes(dest_path, &file)?;
        dbg!("HERE2");
        Ok(())
    }

    fn add_directory(&mut self, path: &Path) -> LivraisonResult<()> {
        let mut header = tar::Header::new_gnu();
        header.set_size(0);
        header.set_mtime(self.mtime);
        header.set_mode(0o755);
        // Lintian insists on dir paths ending with /, which Rust doesn't
        let mut path_str = path.to_string_lossy().to_string();
        if !path_str.ends_with('/') {
            path_str += "/";
        }
        header.set_entry_type(tar::EntryType::Directory);
        header.set_cksum();
        self.builder
            .append_data(&mut header, path_str, &mut io::empty())?;

        Ok(())
    }

    fn add_parent_dirs(&mut self, path: &Path) -> LivraisonResult<()> {
        if let Some(parent) = path.parent() {
            let mut directory = PathBuf::new();
            for comp in parent.components() {
                match comp {
                    Component::Normal(c) => directory.push(c),
                    _ => continue,
                }
                println!("Adding directory: {directory:?}");

                if !self.dirs_added.contains(&directory) {
                    self.dirs_added.insert(directory.clone());
                    self.add_directory(&directory)?;
                }
            }
        }

        Ok(())
    }
}
