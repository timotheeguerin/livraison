use super::{RowView, Table, TableRow, error::MsiDataBaseError};

const TABLE_NAME: &str = "InstallUISequence";

pub struct InstallUISequenceTable {
    pub rows: Vec<InstallUISequenceRow>,
}

pub struct InstallUISequenceRow {
    pub dialog: String,
    pub condition: Option<String>,
    pub order: i32,
}

impl Table<InstallUISequenceRow> for InstallUISequenceTable {
    fn name() -> &'static str {
        TABLE_NAME
    }

    fn new(rows: Vec<InstallUISequenceRow>) -> Self {
        InstallUISequenceTable { rows }
    }
}

impl TableRow for InstallUISequenceRow {
    fn from_row(row: &RowView) -> Result<InstallUISequenceRow, MsiDataBaseError> {
        Ok(InstallUISequenceRow {
            dialog: row.string(0)?,
            condition: row.opt_string(1)?,
            order: row.int(2)?,
        })
    }
}
