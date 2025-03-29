use std::vec;

use msi_installer::{
    tables::{Control, ControlAttributes, ControlEvent, ControlType, Dialog, DialogStyle},
    ui::{self, event::EndDialogAction},
};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("CancelDialog", "[ProductName] Setup")
        .size((260, 85))
        .add(
            ui::control::text("Text", "Do you want to abort [ProductName] [Text_action]?")
                .pos((48, 15))
                .size((194, 30)),
        )
        .add(
            ui::control::button("Cancel", "No")
                .pos((132, 57))
                .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button("Yes", "Yes")
                .pos((72, 57))
                .trigger(ui::event::end_dialog(EndDialogAction::Exit)),
        )
}
pub fn create_cancel_dialog() -> Dialog {
    Dialog {
        dialog: "CancelDialog".to_string(),
        h_centering: 50,
        v_centering: 50,
        width: 260,
        height: 85,
        attributes: DialogStyle::Visible | DialogStyle::Modal,
        title: Some("[ProductName] Setup".to_string()),
        control_first: "No".to_string(),
        control_default: Some("No".to_string()),
        control_cancel: Some("No".to_string()),
    }
}

pub fn create_cancel_dialog_controls() -> Vec<Control> {
    vec![
        Control {
            dialog: "CancelDialog".to_string(),
            control: "No".to_string(),
            type_: ControlType::PushButton,
            x: 132,
            y: 57,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Continue".to_string()),
            control_next: Some("Yes".to_string()),
            help: None,
        },
        Control {
            dialog: "CancelDialog".to_string(),
            control: "Text".to_string(),
            type_: ControlType::Text,
            x: 48,
            y: 15,
            width: 194,
            height: 30,
            attributes: ControlAttributes::NoPrefix
                | ControlAttributes::Visible
                | ControlAttributes::Enabled,
            property: None,
            text: Some("Do you want to abort [ProductName] [Text_action]?".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "CancelDialog".to_string(),
            control: "Yes".to_string(),
            type_: ControlType::PushButton,
            x: 72,
            y: 57,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Abort".to_string()),
            control_next: Some("No".to_string()),
            help: None,
        },
    ]
}

pub fn create_cancel_dialog_control_events() -> Vec<ControlEvent> {
    vec![
        ControlEvent {
            dialog: "CancelDialog".to_string(),
            control: "No".to_string(),
            event: "EndDialog".to_string(),
            argument: "Return".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(15),
        },
        ControlEvent {
            dialog: "CancelDialog".to_string(),
            control: "Yes".to_string(),
            event: "EndDialog".to_string(),
            argument: "Exit".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(16),
        },
    ]
}
