use std::fs;

use crate::{
    LivraisonResult,
    actions::pack::{CommonOptions, LivraisonPacker},
    pkg::{
        metadata::{PkgMetadata, User},
        package::{DataFile, PkgPackage},
    },
};

#[derive(Debug, Default, Clone)]
pub struct PkgLivraisonPacker {}

impl LivraisonPacker for PkgLivraisonPacker {
    fn pack(&self, options: CommonOptions) -> LivraisonResult<()> {
        let name = options.name.clone();
        let metadata = PkgMetadata {
            identifier: format!("com.livraison.{name}"),
            name: name.clone(),
            version: options.version.unwrap_or_else(|| "0.0.0".to_string()),
            install_location: "/".to_string(),
            author: match options.author {
                Some(author) => User {
                    name: author.name,
                    email: author.email,
                },
                None => User::default(),
            },
        };

        let files = options
            .bin_files
            .iter()
            .map(|file| {
                let dest = format!("/usr/local/bin/{}", file.file_name());
                DataFile::new(dest, file.clone().with_mode(0o100755))
            })
            .collect::<Vec<DataFile>>();

        let pkg = PkgPackage {
            metadata: metadata.clone(),
            files,
        };

        let out_file = options.out.join(&name).with_extension("pkg");
        fs::create_dir_all(&options.out)?;

        let file = fs::File::create(&out_file)?;
        pkg.write(file)?;

        println!("Created PKG package at: {}", out_file.to_string_lossy());
        println!("  Identifier: {}", metadata.identifier);
        println!("  Version: {}", metadata.version);
        println!("Included files:");
        if pkg.files.is_empty() {
            println!("  No files included.");
        } else {
            for file in &pkg.files {
                println!("  {}", file.get_dest());
            }
        }
        Ok(())
    }
}
