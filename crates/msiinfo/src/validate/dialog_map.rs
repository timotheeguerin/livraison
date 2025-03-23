use std::collections::{HashMap, hash_map::Values};

use msi_installer::tables::{Control, Dialog, is_standard_action};

pub struct DialogMap {
    pub dialogs: HashMap<String, CompleteDialog>,
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

    pub fn all(&self) -> Values<'_, String, CompleteDialog> {
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

pub struct CompleteDialog {
    pub dialog: Dialog,
    pub controls: HashMap<String, Control>,
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
