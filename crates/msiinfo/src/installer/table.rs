use std::io::{Read, Seek};

use super::TableMissingError;

pub trait TableRow {
    fn from_row(row: &msi::Row) -> Self;
}

pub trait Table<Row: TableRow> {
    fn new(rows: Vec<Row>) -> Self;

    fn from_package<F: Read + Seek>(
        package: &mut msi::Package<F>,
    ) -> Result<Self, TableMissingError>
    where
        Self: std::marker::Sized,
    {
        match package.select_rows(msi::Select::table(Self::name())) {
            Ok(n) => {
                let rows: Vec<Row> = n.map(|row| Self::from_row(&row)).collect();
                Ok(Self::new(rows))
            }
            Err(e) => Err(TableMissingError {
                table: Self::name().to_string(),
            }),
        }
    }

    fn from_row(row: &msi::Row) -> Row {
        Row::from_row(row)
    }

    fn name() -> &'static str;
}
