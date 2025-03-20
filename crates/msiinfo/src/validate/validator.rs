use msi::Package;
use std::io::{Read, Seek};
use thiserror::Error;

use crate::{
    color::{green, red},
    installer::{Dialog, Entity, InstallUISequence, MsiDataBaseError},
};

pub fn validate_msi_installer<F: Read + Seek>(package: &mut Package<F>) {
    match validate_dialogs(package) {
        Ok(errors) => {
            if errors.is_empty() {
                println!("{} No errors found", green("success"));
            } else {
                print_errors(&errors);
            }
        }
        Err(err) => println!("{} {}", red("error"), err),
    }
}

fn print_errors(errors: &Vec<ValidationError>) {
    for error in errors {
        println!("{} {}", red("error"), error);
    }
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("MsiDataBaseError: {0}")]
    MsiDataBaseError(#[from] MsiDataBaseError),

    #[error("Dialog {dialog} referenced in {reference} is missing")]
    MissingDialogError { dialog: String, reference: String },
}

pub type ValidationResult = Result<Vec<ValidationError>, ValidationError>;

fn validate_dialogs<F: Read + Seek>(package: &mut Package<F>) -> ValidationResult {
    let install_ui_sequences = InstallUISequence::list(package)?;
    let dialogs = Dialog::list(package)?;

    let dialog_map = dialogs
        .into_iter()
        .map(|dialog| (dialog.dialog.clone(), dialog))
        .collect::<std::collections::HashMap<String, Dialog>>();

    let mut errors = Vec::new();

    for row in install_ui_sequences.into_iter() {
        if !dialog_map.contains_key(&row.dialog) {
            errors.push(ValidationError::MissingDialogError {
                dialog: row.dialog.clone(),
                reference: InstallUISequence::table_name().to_string(),
            });
        }
    }

    Ok(errors)
}
