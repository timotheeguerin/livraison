use msi_installer::tables::MsiDataBaseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("MsiDataBaseError: {0}")]
    MsiDataBase(#[from] MsiDataBaseError),

    #[error("Dialog {dialog} referenced in {reference} is missing")]
    MissingDialog { dialog: String, reference: String },
    #[error("Control {control} in {dialog} referenced by {reference} is missing")]
    MissingControl {
        dialog: String,
        control: String,
        reference: String,
    },
    #[error(
        "Controls on dialog {dialog} do not form a valid cycle, control {control} is missing a next control"
    )]
    ControlNotCircular { dialog: String, control: String },
    #[error(
        "Control {next_control} on {dialog} is referenced as the next control by multiple dialogs {controls}."
    )]
    DuplicateControlNext {
        dialog: String,
        controls: String,
        next_control: String,
    },

    #[error("Table {table} doesn't have any primary key.")]
    MissingPrimaryKey { table: String },

    #[error(
        "Table {table} column {column} (#{index}) is a primary key but one or more columns defined before is not. Primary keys must be the leading columns of a table."
    )]
    PrimaryKeyLeading {
        table: String,
        column: String,
        index: usize,
    },
}

pub type ValidationResult = Result<Vec<ValidationError>, ValidationError>;
