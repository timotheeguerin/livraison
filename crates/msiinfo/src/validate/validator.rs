use msi::{Package, Row};
use quick_error::quick_error;
use std::io::{Read, Seek};

use crate::{
    color::{green, red},
    installer::{InstallUISequenceTable, TableMissingError},
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

quick_error! {
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum ValidationError {
        TableMissing(err: TableMissingError) {
            from()
            display("Installer is missing table: {}", err.table)
            source(err)
        }

    }
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
