use msi::{Package, Row};
use std::io::{Read, Seek};
use thiserror::Error;

use crate::{
    color::{green, red},
    installer::{DialogTable, InstallUISequenceTable, MsiDataBaseError, Table},
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
    let dialog_table = DialogTable::from_package(package)?;

    for row in install_ui_sequence_table.rows.into_iter() {
        println!(
            "InstallUISequence: {} {:?} {}",
            row.dialog, row.condition, row.order
        );
    }

    for row in dialog_table.rows.into_iter() {
        println!(
            "Dialog: {} {} {} {} {} {} {} {} {} {}",
            row.dialog,
            row.h_centering,
            row.v_centering,
            row.width,
            row.height,
            row.attributes,
            row.title,
            row.control_first,
            row.control_default,
            row.control_cancel
        );
    }

    Ok(())
}
