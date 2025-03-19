use std::io::{Read, Seek};

use super::error::MsiDataBaseError;

pub trait TableRow {
    fn from_row(row: &RowView) -> Result<Self, MsiDataBaseError>
    where
        Self: std::marker::Sized;
}

pub trait Table<Row: TableRow> {
    fn new(rows: Vec<Row>) -> Self;

    fn from_package<F: Read + Seek>(package: &mut msi::Package<F>) -> Result<Self, MsiDataBaseError>
    where
        Self: std::marker::Sized,
    {
        match package.select_rows(msi::Select::table(Self::name())) {
            Ok(n) => {
                let rows: Result<Vec<Row>, MsiDataBaseError> = n
                    .enumerate()
                    .map(|(i, row)| Row::from_row(&RowView::new(Self::name(), &row, i)))
                    .collect();
                Ok(Self::new(rows?))
            }
            Err(_) => Err(MsiDataBaseError::TableMissingError {
                table: Self::name().to_string(),
            }),
        }
    }

    fn name() -> &'static str;
}

pub struct RowView<'a> {
    table: String,
    row_index: usize,
    row: &'a msi::Row,
}

impl<'a> RowView<'a> {
    fn new(table: &str, row: &'a msi::Row, row_index: usize) -> Self {
        RowView {
            table: table.to_string(),
            row,
            row_index,
        }
    }

    pub fn string(&self, index: usize) -> Result<String, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Str(s) => Ok(s.clone()),
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "string".to_string(),
                value: cell.to_string(),
            }),
        }
    }
    pub fn opt_string(&self, index: usize) -> Result<Option<String>, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Str(s) => Ok(Some(s.clone())),
            msi::Value::Null => Ok(None),
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "string".to_string(),
                value: cell.to_string(),
            }),
        }
    }
    pub fn int(&self, index: usize) -> Result<i32, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Int(s) => Ok(s.clone()),
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "int32".to_string(),
                value: cell.to_string(),
            }),
        }
    }
}
