use thiserror::Error;

#[derive(Error, Debug)]
pub enum MsiDataBaseError {
    #[error("Failed to deserialize data: {0}")]
    DeserializationError(String),

    #[error("Table '{table}' is missing in package")]
    TableMissingError { table: String },
    #[error(
        "Table '{table}' cell {row}:{column} is not of the expected type: {expected_type}, value: {value}"
    )]
    CellInvalidTypeError {
        table: String,
        row: usize,
        column: usize,
        expected_type: String,
        value: String,
    },
}
