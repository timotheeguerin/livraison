use std::path::PathBuf;

use crate::{LivraisonResult, msi::MsiLivraisonPacker};

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
    pub author: Option<String>,
}

pub trait LivraisonPacker {
    type Options;

    fn pack(&self, options: CommonOptions) -> LivraisonResult<()>;
}

pub fn pack(packer: impl LivraisonPacker) {}

fn get_packer(target: String) -> impl LivraisonPacker {
    match target.as_str() {
        "msi" => MsiLivraisonPacker {},
        _ => panic!("Unsupported packer for target: {}", target),
    }
}

pub fn pack_for_target(target: String, options: CommonOptions) -> LivraisonResult<()> {
    let packer = get_packer(target);
    packer.pack(options)
}
