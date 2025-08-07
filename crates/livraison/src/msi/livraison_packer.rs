use std::fs;

use super::packer;
use crate::{
    LivraisonResult,
    actions::pack::{CommonOptions, LivraisonPacker},
    msi::packer::MsiInstallerOptions,
};

#[derive(Debug, Default, Clone)]
pub struct MsiLivraisonPacker {}

impl LivraisonPacker for MsiLivraisonPacker {
    fn pack(&self, options: CommonOptions) -> LivraisonResult<()> {
        let out_file = options.out.join(options.name.clone()).with_extension("msi");
        fs::create_dir_all(options.out)?;
        packer::pack(
            MsiInstallerOptions {
                name: options.name.clone(),
                bundle_name: options.name.clone(),
                version: match options.version {
                    Some(v) => v,
                    None => "0.0.0".to_string(),
                },
                description: options.description.unwrap_or("No description.".to_string()),
                author: match options.author {
                    Some(author) => author.name,
                    None => "Unknown".to_string(),
                },
                ..Default::default()
            },
            &out_file,
        )?;
        Ok(())
    }
}
