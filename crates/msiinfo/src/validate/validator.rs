use msi::{Package, Row};
use std::io::{Read, Seek};
use thiserror::Error;

use crate::{
    color::{green, red},
    installer::{InstallUISequenceTable, MsiDataBaseError, Table},
};

struct Error {
    message: String,
}

pub fn validateMsiInstaller<F: Read + Seek>(package: &mut Package<F>) {
    match validate_dialogs(package) {
        Ok(_) => println!("{} No errors found", green("✓")),
        Err(err) => println!("{} {}", red("error"), err),
    }
}

fn printErrors(errors: &Vec<Error>) {
    for error in errors {
        println!("{} {}", red("error"), error.message);
    }
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("MsiDataBaseError: {0}")]
    MsiDataBaseError(#[from] MsiDataBaseError),
}

pub type ValidationResult = Result<(), ValidationError>;

fn validate_dialogs<F: Read + Seek>(package: &mut Package<F>) -> ValidationResult {
    let install_ui_sequence_table = InstallUISequenceTable::from_package(package)?;

    for row in install_ui_sequence_table.rows.into_iter() {
        // Add your validation logic here and push errors to the errors vector
        dbg!(row.dialog, row.condition, row.order);
    }

    Ok(())
}
