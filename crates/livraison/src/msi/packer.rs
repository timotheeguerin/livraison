use std::{
    fs,
    io::{Read, Seek, Write},
    path::Path,
};

use crate::LivraisonResult;
use uuid::Uuid;

// Namespace to construct uuid v5
const UUID_NAMESPACE: Uuid = uuid::uuid!("3941a426-8f68-469a-a7c5-99944d6067d8");

#[derive(Default, Clone, Debug)]
pub struct MsiInstallerOptions {
    /// Unique name that should never change to generate the same UUID
    pub bundle_name: String,

    /// Application name
    pub name: String,

    /// Application version
    pub version: String,

    /// Unique name that should never change to generate the same UUID
    pub description: String,

    /// Author name
    pub author: String,
}

pub struct MsiInstallerPacker<W: Read + Write + Seek> {
    bundle_id: Uuid,
    package: msi::Package<W>,
    options: MsiInstallerOptions,
}

pub fn pack(options: MsiInstallerOptions, dest: &Path) -> LivraisonResult<()> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(dest)?;

    let mut packer = MsiInstallerPacker::new(file, options)?;

    packer.write()?;
    Ok(())
}

impl<W: Read + Write + Seek> MsiInstallerPacker<W> {
    pub fn new(out: W, options: MsiInstallerOptions) -> LivraisonResult<MsiInstallerPacker<W>> {
        let package = msi::Package::create(msi::PackageType::Installer, out)?;
        let bundle_id = compute_bundle_id(&options.bundle_name);

        Ok(MsiInstallerPacker {
            bundle_id,
            package,
            options,
        })
    }

    pub fn write(&mut self) -> LivraisonResult<()> {
        self.set_summary_info();
        self.create_property_table()?;

        self.package.flush()?;
        Ok(())
    }

    // Populates the summary metadata for the package from the bundle settings.
    fn set_summary_info(&mut self) {
        let summary_info = self.package.summary_info_mut();
        summary_info.set_creation_time_to_now();
        summary_info.set_subject(&self.options.name);
        summary_info.set_uuid(self.bundle_id);
        summary_info.set_comments(&self.options.description);
        summary_info.set_author(&self.options.author);
        summary_info.set_creating_application("livraison".to_string());
        summary_info.set_word_count(2);
    }

    // Creates and populates the `Property` database table for the package.
    fn create_property_table(&mut self) -> LivraisonResult<()> {
        self.package.create_table(
            "Property",
            vec![
                msi::Column::build("Property").primary_key().id_string(72),
                msi::Column::build("Value").text_string(0),
            ],
        )?;
        self.package.insert_rows(
            msi::Insert::into("Property")
                .row(vec![
                    msi::Value::from("ProductCode"),
                    msi::Value::from(self.bundle_id),
                ])
                .row(vec![
                    msi::Value::from("ProductLanguage"),
                    msi::Value::from(msi::Language::from_tag("en-US")),
                ])
                .row(vec![
                    msi::Value::from("Manufacturer"),
                    msi::Value::Str(self.options.author.clone()),
                ])
                .row(vec![
                    msi::Value::from("ProductName"),
                    msi::Value::from(self.options.name.clone()),
                ])
                .row(vec![
                    msi::Value::from("ProductVersion"),
                    msi::Value::from(self.options.version.clone()),
                ]), // .row(vec![
                    //     msi::Value::from("DefaultUIFont"),
                    //     msi::Value::from("DefaultFont"),
                    // ])
                    // .row(vec![msi::Value::from("Mode"), msi::Value::from("Install")])
                    // .row(vec![
                    //     msi::Value::from("Text_action"),
                    //     msi::Value::from("installation"),
                    // ])
                    // .row(vec![
                    //     msi::Value::from("Text_agent"),
                    //     msi::Value::from("installer"),
                    // ])
                    // .row(vec![
                    //     msi::Value::from("Text_Doing"),
                    //     msi::Value::from("installing"),
                    // ])
                    // .row(vec![
                    //     msi::Value::from("Text_done"),
                    //     msi::Value::from("installed"),
                    // ]),
        )?;
        Ok(())
    }
}

fn compute_bundle_id(bundle_name: &str) -> uuid::Uuid {
    Uuid::new_v5(&UUID_NAMESPACE, bundle_name.as_bytes())
}
