use super::{Entity, RowView, error::MsiDataBaseError};

/// Directory Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/directory-table
#[derive(Debug, Clone)]
pub struct Directory {
    pub directory: String,
    pub parent: Option<String>,
    pub default_dir: String,
}

impl Entity for Directory {
    fn table_name() -> &'static str {
        "Directory"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Directory").primary_key().id_string(72),
            msi::Column::build("Directory_Parent")
                .nullable()
                .foreign_key("Directory", 1)
                .id_string(72),
            msi::Column::build("DefaultDir")
                .category(msi::Category::DefaultDir)
                .localizable()
                .string(255),
        ]
    }

    fn from_row(row: &RowView) -> Result<Directory, MsiDataBaseError> {
        Ok(Directory {
            directory: row.string(0)?,
            parent: row.opt_string(1)?,
            default_dir: row.string(2)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.directory.clone()),
            msi::Value::from_opt_string(&self.parent),
            msi::Value::Str(self.default_dir.clone()),
        ]
    }
}
