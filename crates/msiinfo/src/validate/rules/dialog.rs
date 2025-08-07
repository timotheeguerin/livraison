use crate::validate::{
    dialog_map::{CompleteDialog, DialogMap},
    rule::{Rule, RuleContext, RuleData},
};
use msi_installer::tables::{Control, Entity, InstallUISequence};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub struct DialogRule {}
impl Rule for DialogRule {
    fn code(&self) -> &'static str {
        "invalid-dialog"
    }

    fn run(
        &self,
        ctx: &mut RuleContext,
        data: &RuleData,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let RuleData {
            controls,
            dialog_map,
            install_ui_sequences,
            ..
        } = data;

        let mut errors = Vec::new();
        // Validate install_ui_sequences dialog are referenced.
        for row in install_ui_sequences {
            if !dialog_map.has_dialog_or_std_action(&row.dialog) {
                errors.push(ErrorMsg::MissingDialog {
                    dialog: row.dialog.clone(),
                    reference: InstallUISequence::table_name().to_string(),
                });
            }
        }

        for complete_dialog in dialog_map.all() {
            let dialog = &complete_dialog.dialog;
            if !complete_dialog.has_control(&dialog.control_first) {
                errors.push(ErrorMsg::MissingControl {
                    dialog: dialog.dialog.clone(),
                    control: dialog.control_first.clone(),
                    reference: format!("control_first of dialog {}", dialog.dialog),
                });
            }
            if let Some(control_default) = &dialog.control_default
                && !complete_dialog.has_control(control_default)
            {
                errors.push(ErrorMsg::MissingControl {
                    dialog: dialog.dialog.clone(),
                    control: control_default.clone(),
                    reference: format!("control_default of dialog {}", dialog.dialog),
                });
            }
            if let Some(control_cancel) = &dialog.control_cancel
                && !complete_dialog.has_control(control_cancel)
            {
                errors.push(ErrorMsg::MissingControl {
                    dialog: dialog.dialog.clone(),
                    control: control_cancel.clone(),
                    reference: format!("control_cancel of dialog {}", dialog.dialog),
                });
            }
        }

        for row in controls {
            let dialog = match dialog_map.get(&row.dialog) {
                Some(dialog) => dialog,
                None => {
                    errors.push(ErrorMsg::MissingDialog {
                        dialog: row.dialog.clone(),
                        reference: Control::table_name().to_string(),
                    });
                    continue;
                }
            };

            if let Some(next_control) = &row.control_next
                && !dialog.has_control(next_control)
            {
                errors.push(ErrorMsg::MissingControl {
                    dialog: row.dialog.clone(),
                    control: next_control.clone(),
                    reference: format!("next_control of {}", row.control),
                });
            }
        }
        errors.extend(validate_dialogs_control_order(dialog_map));

        for error in errors {
            ctx.error(error.to_string());
        }
        Ok(())
    }
}

// windows installer error 2809 and 2810
fn validate_dialogs_control_order(dialog_map: &DialogMap) -> Vec<ErrorMsg> {
    fn visit(
        control: &Control,
        complete_dialog: &CompleteDialog,
        visited: &mut HashSet<String>,
    ) -> Vec<ErrorMsg> {
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
            vec![ErrorMsg::ControlNotCircular {
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
                errors.push(ErrorMsg::DuplicateControlNext {
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

#[derive(Error, Debug)]
pub enum ErrorMsg {
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
