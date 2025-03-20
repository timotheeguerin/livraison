use super::{Entity, RowView, error::MsiDataBaseError};

const TABLE_NAME: &str = "InstallUISequence";

pub struct InstallUISequence {
    pub dialog: String,
    pub condition: Option<String>,
    pub order: i32,
}

impl Entity for InstallUISequence {
    fn table_name() -> &'static str {
        TABLE_NAME
    }

    fn from_row(row: &RowView) -> Result<InstallUISequence, MsiDataBaseError> {
        Ok(InstallUISequence {
            dialog: row.string(0)?,
            condition: row.opt_string(1)?,
            order: row.i32(2)?,
        })
    }
}
