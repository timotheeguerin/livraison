use msi_installer::tables::{Control, ControlEvent, Dialog, DialogStyle};

pub fn create_cancel_dialog() -> Dialog {
    Dialog {
        dialog: "CancelDialog".to_string(),
        h_centering: 50,
        v_centering: 10,
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
            type_: "PushButton".to_string(),
            x: 132,
            y: 57,
            width: 56,
            height: 17,
            attributes: 3,
            property: None,
            text: Some("Continue".to_string()),
            control_next: Some("Yes".to_string()),
            help: None,
        },
        Control {
            dialog: "CancelDialog".to_string(),
            control: "Text".to_string(),
            type_: "Text".to_string(),
            x: 48,
            y: 15,
            width: 194,
            height: 30,
            attributes: 131075,
            property: None,
            text: Some("Do you want to abort [ProductName] [Text_action]?".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "CancelDialog".to_string(),
            control: "Yes".to_string(),
            type_: "PushButton".to_string(),
            x: 72,
            y: 57,
            width: 56,
            height: 17,
            attributes: 3,
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
