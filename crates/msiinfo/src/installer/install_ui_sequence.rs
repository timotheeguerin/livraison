use std::io::{Read, Seek};

use super::{Table, TableMissingError, TableRow};

const TABLE_NAME: &str = "InstallUISequence";

pub struct InstallUISequenceTable {
    pub rows: Vec<InstallUISequenceRow>,
}

pub struct InstallUISequenceRow {
    pub dialog: String,
    pub condition: String,
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
    fn from_row(row: &msi::Row) -> InstallUISequenceRow {
        InstallUISequenceRow {
            dialog: row[0].to_string(),
            condition: row[1].to_string(),
            order: row[2].to_string().parse().unwrap(),
        }
    }
}
