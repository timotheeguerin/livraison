use std::io::{Read, Seek};

use super::TableMissingError;

const TABLE_NAME: &str = "InstallUISequence";

pub struct InstallUISequenceTable {
    pub rows: Vec<InstallUISequenceRow>,
}

pub struct InstallUISequenceRow {
    pub dialog: String,
    pub condition: String,
    pub order: i32,
}

impl InstallUISequenceTable {
    pub fn from_package<F: Read + Seek>(
        package: &mut msi::Package<F>,
    ) -> Result<InstallUISequenceTable, TableMissingError> {
        match package.select_rows(msi::Select::table(TABLE_NAME)) {
            Ok(n) => {
                let rows: Vec<InstallUISequenceRow> =
                    n.map(|row| InstallUISequenceRow::from_row(&row)).collect();
                Ok(InstallUISequenceTable { rows })
            }
            Err(e) => Err(TableMissingError {
                table: TABLE_NAME.to_string(),
            }),
        }
    }
}

impl InstallUISequenceRow {
    pub fn from_row(row: &msi::Row) -> InstallUISequenceRow {
        InstallUISequenceRow {
            dialog: row[0].to_string(),
            condition: row[1].to_string(),
            order: row[2].to_string().parse().unwrap(),
        }
    }
}
