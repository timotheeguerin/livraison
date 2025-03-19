use super::{RowView, Table, TableRow, error::MsiDataBaseError};

const TABLE_NAME: &str = "InstallUISequence";

pub struct InstallUISequenceRow {
    pub dialog: String,
    pub condition: Option<String>,
    pub order: i32,
}

pub type InstallUISequenceTable = Table<InstallUISequenceRow>;

impl TableRow for InstallUISequenceRow {
    fn table_name() -> &'static str {
        TABLE_NAME
    }

    fn from_row(row: &RowView) -> Result<InstallUISequenceRow, MsiDataBaseError> {
        Ok(InstallUISequenceRow {
            dialog: row.string(0)?,
            condition: row.opt_string(1)?,
            order: row.i32(2)?,
        })
    }
}
