//! Assembly of a complete macOS flat *component* `.pkg` file: a xar archive
//! containing `Bom`, `PackageInfo` and a gzip-compressed cpio `Payload`.

use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    LivraisonResult,
    common::FileRef,
    pkg::{
        bom::BomBuilder,
        metadata::PkgMetadata,
        package_info,
        xar::{self, XarFile},
    },
    utils::{compression::gzip, cpio::CpioBuilder},
};

const DIR_MODE: u32 = 0o40755;

/// A file to include in the package, installed at `dest` (an absolute path).
#[derive(Debug, Clone)]
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
}

/// A full macOS component package ready to be serialized.
pub struct PkgPackage {
    pub metadata: PkgMetadata,
    pub files: Vec<DataFile>,
}

/// Given absolute payload paths, return the set of ancestor directories
/// (relative, without a leading slash) ordered parents-first, e.g.
/// `["usr", "usr/local", "usr/local/bin"]`.
fn ancestor_dirs(dests: &[String]) -> Vec<String> {
    let mut dirs = BTreeSet::new();
    for dest in dests {
        let comps: Vec<&str> = dest.split('/').filter(|c| !c.is_empty()).collect();
        // Every component except the last (the file itself) is a directory.
        for i in 1..comps.len() {
            dirs.insert(comps[..i].join("/"));
        }
    }
    // BTreeSet iterates lexicographically, which places parents before their
    // children (a parent path is a prefix of, and shorter than, its children).
    dirs.into_iter().collect()
}

impl PkgPackage {
    pub fn write<W: Write>(&self, mut out: W) -> LivraisonResult<()> {
        let mtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);

        let dests: Vec<String> = self.files.iter().map(|f| f.dest.clone()).collect();
        let dirs = ancestor_dirs(&dests);

        // --- Payload (cpio) and Bom share the same file tree. ---
        let mut cpio = CpioBuilder::new_odc();
        let mut bom = BomBuilder::new();

        cpio.add_dir(".", DIR_MODE, mtime);
        for dir in &dirs {
            cpio.add_dir(&format!("./{dir}"), DIR_MODE, mtime);
        }

        let mut total_bytes: u64 = 0;
        for file in &self.files {
            let mut data = Vec::new();
            file.source.open()?.read_to_end(&mut data)?;
            total_bytes += data.len() as u64;

            let mode = as_file_mode(file.source.get_mode().unwrap_or(0o100755));
            let cpio_path = format!(".{}", file.dest);
            cpio.add_file_bytes(&cpio_path, mode, mtime, &data);
            bom.add_file(&file.dest, mode, 0, 0, mtime, &data);
        }

        let payload = gzip(&cpio.finish())?;
        let bom_bytes = crate::pkg::bom::build(&bom);

        // numberOfFiles counts every payload entry: "." + directories + files.
        let num_files = 1 + dirs.len() as u32 + self.files.len() as u32;
        let install_kb = total_bytes.div_ceil(1024);
        let package_info = package_info::render(&self.metadata, num_files, install_kb);

        let xar_bytes = xar::build(&[
            XarFile {
                name: "Bom".to_string(),
                data: bom_bytes,
            },
            XarFile {
                name: "PackageInfo".to_string(),
                data: package_info.into_bytes(),
            },
            XarFile {
                name: "Payload".to_string(),
                data: payload,
            },
        ])?;

        out.write_all(&xar_bytes)?;
        Ok(())
    }
}

/// Ensure a mode has the regular-file type bits set.
fn as_file_mode(mode: u32) -> u32 {
    if mode & 0o170000 == 0 {
        mode | 0o100000
    } else {
        mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ancestor_dirs_are_parent_first() {
        let dirs = ancestor_dirs(&["/usr/local/bin/foo".to_string()]);
        assert_eq!(dirs, vec!["usr", "usr/local", "usr/local/bin"]);
    }

    #[test]
    fn produces_a_xar_archive() {
        let pkg = PkgPackage {
            metadata: PkgMetadata::default(),
            files: vec![DataFile::new(
                "/usr/local/bin/foo",
                FileRef::from_text("#!/bin/sh\n").with_mode(0o100755),
            )],
        };
        let mut buf = Vec::new();
        pkg.write(&mut buf).unwrap();
        assert_eq!(&buf[0..4], b"xar!");
    }
}
