use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct TableMissingError {
    pub table: String,
}

impl fmt::Display for TableMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table '{}' is missing in package", self.table)
    }
}
impl Error for TableMissingError {}
