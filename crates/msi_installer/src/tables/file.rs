use super::{Entity, RowView, error::MsiDataBaseError};
use bitflags::bitflags;

/// Control Event Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/file-table
#[derive(Debug, Clone, Default)]
pub struct File {
    pub file: String,
    pub component: String,
    pub filename: String,
    pub size: i32,
    pub version: Option<String>,
    pub language: Option<String>,
    pub attributes: FileAttributes,
    pub sequence: i32,
}

impl Entity for File {
    fn table_name() -> &'static str {
        "File"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("File").primary_key().id_string(72),
            msi::Column::build("Component_")
                .foreign_key("Component", 1)
                .id_string(72),
            msi::Column::build("FileName")
                .category(msi::Category::Filename)
                .string(255),
            msi::Column::build("FileSize").range(0, 0x7fffffff).int32(),
            msi::Column::build("Version")
                .nullable()
                .category(msi::Category::Version)
                .string(72),
            msi::Column::build("Language")
                .nullable()
                .category(msi::Category::Language)
                .string(20),
            msi::Column::build("Attributes")
                .nullable()
                .range(0, 0x7fff)
                .int16(),
            msi::Column::build("Sequence").range(1, 0x7fff).int16(),
        ]
    }

    fn from_row(row: &RowView) -> Result<File, MsiDataBaseError> {
        Ok(File {
            file: row.string(0)?,
            component: row.string(1)?,
            filename: row.string(2)?,
            size: row.i32(3)?,
            version: row.opt_string(4)?,
            language: row.opt_string(5)?,
            attributes: FileAttributes::from_bits_retain(row.i32(6)?),
            sequence: row.i32(7)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.file.clone()),
            msi::Value::Str(self.component.clone()),
            msi::Value::Str(self.filename.clone()),
            msi::Value::Int(self.size),
            msi::Value::from_opt_string(&self.version),
            msi::Value::from_opt_string(&self.language),
            msi::Value::Int(self.attributes.bits()),
            msi::Value::Int(self.sequence),
        ]
    }
}

bitflags! {
    /// File Attributes
    /// https://learn.microsoft.com/en-us/windows/win32/msi/file-table#Attributes
    #[derive(Debug, Clone, Default)]
    pub struct FileAttributes: i32 {
        const ReadOnly = 1;
        const Hidden = 2;
        const System = 4;
        /// The file is vital for the accurate operation of the component to which it belongs.
        /// If the installation of a file with the msidbFileAttributesVital attribute fails, the installation stops and is rolled back.
        /// In this case, the Installer displays a dialog box without an Ignore button.
        /// If this attribute is not set, and the installation of the file fails, the Installer displays a dialog box with an Ignore button.
        /// In this case, the user can choose to ignore the failure to install the file and continue.
        const Vital = 512;
        /// The file contains a valid checksum. A checksum is required to repair a file that has become corrupted.
        const Checksum = 1024;
        /// This bit must only be added by a patch and if the file is being added by the patch.
        const PatchAdded = 4096;
        /// The file's source type is uncompressed. If set, ignore the Word Count Summary Property. If neither msidbFileAttributesNoncompressed or msidbFileAttributesCompressed are set, the compression state of the file is specified by the Word Count Summary Property. Do not set both msidbFileAttributesNoncompressed and msidbFileAttributesCompressed.
        const NonCompressed = 8192;
        /// The file's source type is compressed. If set, ignore the Word Count Summary Property. If neither msidbFileAttributesNoncompressed or msidbFileAttributesCompressed are set, the compression state of the file is specified by the Word Count Summary Property. Do not set both msidbFileAttributesNoncompressed and msidbFileAttributesCompressed.
        const Compressed = 16384;
    }
}
