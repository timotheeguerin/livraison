use std::io::{Read, Seek, Write};

use uuid::Uuid;

use super::error::MsiDataBaseError;

pub trait Entity
where
    Self: std::marker::Sized,
{
    fn table_name() -> &'static str;
    fn definition() -> Vec<msi::Column>;
    fn from_row(row: &RowView) -> Result<Self, MsiDataBaseError>;
    fn to_row(&self) -> Vec<msi::Value>;

    fn list<F: Read + Seek>(package: &mut msi::Package<F>) -> Result<Vec<Self>, MsiDataBaseError> {
        let table_name = Self::table_name();
        match package.select_rows(msi::Select::table(table_name)) {
            Ok(n) => n
                .enumerate()
                .map(|(i, row)| Self::from_row(&RowView::new(table_name, &row, i)))
                .collect(),
            Err(_) => Err(MsiDataBaseError::TableMissingError {
                table: table_name.to_string(),
            }),
        }
    }

    fn create_table<F: Read + Write + Seek>(
        package: &mut msi::Package<F>,
    ) -> Result<(), std::io::Error> {
        let table_name = Self::table_name();
        let columns = Self::definition();
        package.create_table(table_name, columns)
    }

    fn insert<F: Read + Write + Seek>(
        package: &mut msi::Package<F>,
        items: &[Self],
    ) -> Result<(), std::io::Error> {
        let rows = items.iter().map(|item| item.to_row()).collect::<Vec<_>>();
        package.insert_rows(msi::Insert::into(Self::table_name()).rows(rows))?;
        Ok(())
    }
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

    pub fn opt_i32(&self, index: usize) -> Result<Option<i32>, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Int(s) => Ok(Some(*s)),
            msi::Value::Null => Ok(None),
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "int32".to_string(),
                value: cell.to_string(),
            }),
        }
    }

    pub fn i32(&self, index: usize) -> Result<i32, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Int(s) => Ok(*s),
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "int32".to_string(),
                value: cell.to_string(),
            }),
        }
    }

    pub fn i16(&self, index: usize) -> Result<i16, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Int(s) => Ok(*s as i16),
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "int32".to_string(),
                value: cell.to_string(),
            }),
        }
    }

    pub fn opt_uuid(&self, index: usize) -> Result<Option<Uuid>, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Str(s) => match Uuid::parse_str(s) {
                Ok(uuid) => Ok(Some(uuid)),
                Err(e) => Err(MsiDataBaseError::DeserializationError(e.to_string())),
            },
            msi::Value::Null => Ok(None),
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "uuid".to_string(),
                value: cell.to_string(),
            }),
        }
    }
    pub fn uuid(&self, index: usize) -> Result<Uuid, MsiDataBaseError> {
        let cell = &self.row[index];
        match cell {
            msi::Value::Str(s) => match Uuid::parse_str(s) {
                Ok(uuid) => Ok(uuid),
                Err(e) => Err(MsiDataBaseError::DeserializationError(e.to_string())),
            },
            _ => Err(MsiDataBaseError::CellInvalidTypeError {
                table: self.table.clone(),
                row: self.row_index,
                column: index,
                expected_type: "uuid".to_string(),
                value: cell.to_string(),
            }),
        }
    }
}
