use std::fs;

use crate::{
    LivraisonResult,
    actions::pack::{CommonOptions, LivraisonPacker},
    deb::{
        control::{Control, User},
        package::{DataFile, DebPackage},
    },
};

#[derive(Debug, Default, Clone)]
pub struct DebLivraisonPacker {}

impl LivraisonPacker for DebLivraisonPacker {
    fn pack(&self, options: CommonOptions) -> LivraisonResult<()> {
        let control = Control {
            package: options.name.clone(),
            version: options.version.unwrap_or("1.0.0".to_string()),
            revision: Some("12".to_string()),
            description: options.description.unwrap_or("No description.".to_string()),
            architecture: "all".to_string(),
            maintainer: match options.author {
                Some(author) => User {
                    name: author.name,
                    email: author.email,
                },
                None => User {
                    name: "Unknown".to_string(),
                    email: "unknown@unknown.com".to_string(),
                },
            },
            ..Default::default()
        };

        let pkg = DebPackage {
            control: control.clone(),
            files: Some(
                options
                    .bin_files
                    .iter()
                    .map(|file| {
                        let dest = format!("/usr/local/bin/{}", file.file_name());
                        DataFile::new(dest, file.clone())
                    })
                    .collect::<Vec<DataFile>>(),
            ),
            conf_files: None,
        };
        let out_file = options.out.join(options.name.clone()).with_extension("deb");
        fs::create_dir_all(options.out)?;

        let file = fs::File::create(&out_file)?;
        pkg.write(file)?;

        println!("Created DEB package at: {}", out_file.to_string_lossy());
        println!("Control: {:#?}", control.write());
        println!("Included files:");
        if let Some(files) = &pkg.files {
            for file in files {
                println!(" {:#?}", &file.get_dest());
            }
        } else {
            println!(" No files included.");
        }
        Ok(())
    }
}
