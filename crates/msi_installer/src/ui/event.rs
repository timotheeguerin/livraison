use crate::tables::{ControlEvent, ControlEventType};

/// Create a new dialog event. Replace the current dialog with a new one.
pub fn new_dialog(name: &str) -> Event {
    Event {
        event: ControlEventType::NewDialog,
        argument: name.to_string(),
    }
}

// Spawn a new child dialog
pub fn spawn_dialog(name: &str) -> Event {
    Event {
        event: ControlEventType::NewDialog,
        argument: name.to_string(),
    }
}

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
pub enum EndDialogAction {
    Exit,
    Retry,
    Ignore,
    Return,
}

// Notifies the installer to remove a modal dialog box.
pub fn end_dialog(name: &EndDialogAction) -> Event {
    Event {
        event: ControlEventType::NewDialog,
        argument: name.to_string(),
    }
}

#[derive(Debug)]
pub struct Event {
    event: ControlEventType,
    argument: String,
}

impl Event {
    pub fn as_control_event(&self, dialog: &str, control: &str, ordering: i32) -> ControlEvent {
        ControlEvent {
            dialog: dialog.to_string(),
            control: control.to_string(),
            event: self.event.to_string(),
            argument: self.argument.clone(),
            condition: Some("1".to_string()),
            ordering: Some(ordering),
        }
    }
}
