use crate::color::{green, red};
use msi::Package;
use msi_installer::tables::{
    Control, Dialog, Entity, InstallUISequence, MsiDataBaseError, is_standard_action,
};
use std::{
    collections::{HashMap, HashSet, hash_map::Values},
    io::{Read, Seek},
};
use thiserror::Error;

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
}

pub type ValidationResult = Result<Vec<ValidationError>, ValidationError>;

fn validate_dialogs<F: Read + Seek>(package: &mut Package<F>) -> ValidationResult {
    let install_ui_sequences = InstallUISequence::list(package)?;
    let dialogs = Dialog::list(package)?;
    let controls = Control::list(package)?;

    let dialog_map = DialogMap::new(dialogs.clone(), Control::list(package)?);

    let mut errors = Vec::new();

    // Validate install_ui_sequences dialog are referenced.
    for row in install_ui_sequences {
        if !dialog_map.has_dialog_or_std_action(&row.dialog) {
            errors.push(ValidationError::MissingDialog {
                dialog: row.dialog.clone(),
                reference: InstallUISequence::table_name().to_string(),
            });
        }
    }

    for complete_dialog in dialog_map.all() {
        let dialog = &complete_dialog.dialog;
        if !complete_dialog.has_control(&dialog.control_first) {
            errors.push(ValidationError::MissingControl {
                dialog: dialog.dialog.clone(),
                control: dialog.control_first.clone(),
                reference: format!("control_first of dialog {}", dialog.dialog),
            });
        }
        if let Some(control_default) = &dialog.control_default {
            if !complete_dialog.has_control(control_default) {
                errors.push(ValidationError::MissingControl {
                    dialog: dialog.dialog.clone(),
                    control: control_default.clone(),
                    reference: format!("control_default of dialog {}", dialog.dialog),
                });
            }
        }
        if let Some(control_cancel) = &dialog.control_cancel {
            if !complete_dialog.has_control(control_cancel) {
                errors.push(ValidationError::MissingControl {
                    dialog: dialog.dialog.clone(),
                    control: control_cancel.clone(),
                    reference: format!("control_cancel of dialog {}", dialog.dialog),
                });
            }
        }
    }

    for row in controls {
        let dialog = match dialog_map.get(&row.dialog) {
            Some(dialog) => dialog,
            None => {
                errors.push(ValidationError::MissingDialog {
                    dialog: row.dialog.clone(),
                    reference: Control::table_name().to_string(),
                });
                continue;
            }
        };

        if let Some(next_control) = row.control_next {
            if !dialog.has_control(&next_control) {
                errors.push(ValidationError::MissingControl {
                    dialog: row.dialog.clone(),
                    control: next_control.clone(),
                    reference: format!("next_control of {}", row.control),
                });
            }
        }
    }
    errors.extend(validate_dialogs_control_order(&dialog_map));

    Ok(errors)
}

// windows installer error 2809 and 2810
fn validate_dialogs_control_order(dialog_map: &DialogMap) -> Vec<ValidationError> {
    fn visit(
        control: &Control,
        complete_dialog: &CompleteDialog,
        visited: &mut HashSet<String>,
    ) -> Vec<ValidationError> {
        if visited.contains(&control.control) {
            return vec![]; // all good we found a cycle
        }
        visited.insert(control.control.clone());

        if let Some(next_control_name) = control.control_next.as_ref() {
            if let Some(next_control) = complete_dialog.get_control(next_control_name) {
                visit(next_control, complete_dialog, visited)
            } else {
                // ignore missing control error, it will be caught by the missing control validator
                vec![]
            }
        } else {
            vec![ValidationError::ControlNotCircular {
                dialog: control.dialog.clone(),
                control: control.control.clone(),
            }]
        }
    }
    let mut errors = Vec::new();
    for complete_dialog in dialog_map.all() {
        // Contains control visited that have a next control.
        let mut visited = HashSet::<String>::new();

        for control in complete_dialog.list_controls() {
            if visited.contains(&control.control) {
                continue;
            }
            if control.control_next.is_some() {
                errors.extend(visit(control, complete_dialog, &mut visited));
            }
        }
        let mut dup = DuplicateTracker::new();

        for control in complete_dialog.list_controls() {
            if let Some(next_control_name) = control.control_next.as_ref() {
                dup.track(next_control_name.clone(), control.control.clone());
            }
        }

        for (next_control_name, controls) in dup.duplicates() {
            if controls.len() > 1 {
                errors.push(ValidationError::DuplicateControlNext {
                    dialog: complete_dialog.dialog.dialog.clone(),
                    controls: controls.join(", "),
                    next_control: next_control_name.clone(),
                });
            }
        }
    }

    errors
}

struct DuplicateTracker<T> {
    map: HashMap<String, Vec<T>>,
}

impl<T> DuplicateTracker<T> {
    pub fn new() -> Self {
        DuplicateTracker {
            map: HashMap::new(),
        }
    }

    pub fn track(&mut self, key: String, value: T) {
        self.map.entry(key).or_default().push(value);
    }

    pub fn duplicates(&self) -> &HashMap<String, Vec<T>> {
        &self.map
    }
}

struct DialogMap {
    dialogs: HashMap<String, CompleteDialog>,
}

impl DialogMap {
    pub fn new(dialogs: Vec<Dialog>, controls: Vec<Control>) -> Self {
        let mut dialog_map = HashMap::<String, CompleteDialog>::new();
        for dialog in dialogs.into_iter() {
            let controls = controls
                .clone()
                .into_iter()
                .filter(|control| control.dialog == dialog.dialog)
                .map(|control| (control.control.clone(), control))
                .collect::<HashMap<String, Control>>();
            dialog_map.insert(dialog.dialog.clone(), CompleteDialog { dialog, controls });
        }

        DialogMap {
            dialogs: dialog_map,
        }
    }

    fn all(&self) -> Values<'_, String, CompleteDialog> {
        self.dialogs.values()
    }

    pub fn get(&self, dialog: &str) -> Option<&CompleteDialog> {
        self.dialogs.get(dialog)
    }

    pub fn has_dialog(&self, name: &str) -> bool {
        self.dialogs.contains_key(name)
    }
    pub fn has_dialog_or_std_action(&self, name: &str) -> bool {
        self.dialogs.contains_key(name) || is_standard_action(name)
    }
}

struct CompleteDialog {
    dialog: Dialog,
    controls: HashMap<String, Control>,
}

impl CompleteDialog {
    pub fn has_control(&self, name: &str) -> bool {
        self.controls.contains_key(name)
    }

    pub fn get_control(&self, name: &str) -> Option<&Control> {
        self.controls.get(name)
    }
    pub fn list_controls(&self) -> Values<'_, String, Control> {
        self.controls.values()
    }
}
