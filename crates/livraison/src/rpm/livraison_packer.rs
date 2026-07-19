use std::fs;

use crate::{
    LivraisonResult,
    actions::pack::{CommonOptions, LivraisonPacker},
    rpm::{
        metadata::{RpmMetadata, User},
        package::{DataFile, RpmPackage},
    },
};

#[derive(Debug, Default, Clone)]
pub struct RpmLivraisonPacker {}

impl LivraisonPacker for RpmLivraisonPacker {
    fn pack(&self, options: CommonOptions) -> LivraisonResult<()> {
        let description = options
            .description
            .clone()
            .unwrap_or_else(|| "No description.".to_string());

        let metadata = RpmMetadata {
            name: options.name.clone(),
            version: options.version.unwrap_or_else(|| "1.0.0".to_string()),
            release: "1".to_string(),
            // RPM requires a non-empty summary; fall back to the first line of
            // the description.
            summary: description.lines().next().unwrap_or("").to_string(),
            description,
            license: "Unknown".to_string(),
            arch: "noarch".to_string(),
            packager: match options.author {
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

        let pkg = RpmPackage {
            metadata: metadata.clone(),
            files,
        };

        let out_file = options.out.join(&options.name).with_extension("rpm");
        fs::create_dir_all(&options.out)?;

        let file = fs::File::create(&out_file)?;
        pkg.write(file)?;

        println!("Created RPM package at: {}", out_file.to_string_lossy());
        println!("  Name: {}", metadata.name);
        println!("  Version: {}-{}", metadata.version, metadata.release);
        println!("  Arch: {}", metadata.arch);
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
