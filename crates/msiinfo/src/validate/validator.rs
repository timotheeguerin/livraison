use msi::Package;
use std::{
    collections::HashMap,
    io::{Read, Seek},
};
use thiserror::Error;

use crate::{
    color::{green, red},
    installer::{Control, Dialog, Entity, InstallUISequence, MsiDataBaseError, is_standard_action},
};

pub fn validate_msi_installer<F: Read + Seek>(package: &mut Package<F>) {
    match validate_dialogs(package) {
        Ok(errors) => {
            if errors.is_empty() {
                println!("{} No errors found", green("✓"));
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
    #[error("Control {control} in {dialog} referenced by {reference} is missing")]
    MissingControlError {
        dialog: String,
        control: String,
        reference: String,
    },
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

    let dialog_exists =
        |dialog: &str| -> bool { dialog_map.contains_key(dialog) || is_standard_action(dialog) };
    for row in install_ui_sequences.into_iter() {
        if !dialog_exists(&row.dialog) {
            errors.push(ValidationError::MissingDialogError {
                dialog: row.dialog.clone(),
                reference: InstallUISequence::table_name().to_string(),
            });
        }
    }

    let controls = Control::list(package)?;
    let mut control_map = HashMap::<String, HashMap<String, Control>>::new();
    // TODO: this clone feels expensive
    for row in controls.clone().into_iter() {
        if !control_map.contains_key(&row.dialog) {
            control_map.insert(row.dialog.clone(), HashMap::new());
        }
        control_map
            .get_mut(&row.dialog)
            .unwrap()
            .insert(row.control.clone(), row.clone());
    }

    for row in controls.into_iter() {
        if !dialog_exists(&row.dialog) {
            errors.push(ValidationError::MissingDialogError {
                dialog: row.dialog.clone(),
                reference: Control::table_name().to_string(),
            });
        }

        if let Some(next_control) = row.control_next {
            if !control_map
                .get(&row.dialog)
                .unwrap()
                .contains_key(&next_control)
            {
                errors.push(ValidationError::MissingControlError {
                    dialog: row.dialog.clone(),
                    control: next_control.clone(),
                    reference: format!("next_control of {}", row.control),
                });
            }
        }
    }

    Ok(errors)
}
