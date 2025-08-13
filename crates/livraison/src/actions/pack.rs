use std::path::PathBuf;

use crate::{LivraisonResult, common::FileRef, deb::DebLivraisonPacker, msi::MsiLivraisonPacker};

#[derive(Debug, Default, Clone)]
pub struct CommonOptions {
    /// Name of the bundle
    pub name: String,

    /// Output file path
    pub out: PathBuf,

    /// Product version
    pub version: Option<String>,

    /// Product description
    pub description: Option<String>,

    /// Product author
    pub author: Option<User>,

    /// Binary files
    pub bin_files: Vec<FileRef>,
}

#[derive(Debug, Default, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

pub trait LivraisonPacker {
    fn pack(&self, options: CommonOptions) -> LivraisonResult<()>;
}

fn get_packer(target: String) -> Box<dyn LivraisonPacker> {
    match target.as_str() {
        "msi" => Box::new(MsiLivraisonPacker {}),
        "deb" => Box::new(DebLivraisonPacker {}),
        _ => panic!("Unsupported packer for target: {}", target),
    }
}

pub fn pack_for_target(target: String, options: CommonOptions) -> LivraisonResult<()> {
    let packer = get_packer(target);
    packer.pack(options)
}
