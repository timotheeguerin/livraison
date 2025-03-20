use msi_installer::tables::{Control, Dialog};

pub fn create_cancel_dialog() -> Dialog {
    Dialog {
        dialog: "CancelDialog".to_string(),
        h_centering: 50,
        v_centering: 10,
        width: 260,
        height: 85,
        attributes: 3,
        title: Some("[ProductName] Setup".to_string()),
        control_first: "CancelNo".to_string(),
        control_default: Some("CancelNo".to_string()),
        control_cancel: Some("CancelNo".to_string()),
    }
}

pub fn create_cancel_dialog_controls() -> Vec<Control> {
    vec![
        Control {
            dialog: "CancelDialog".to_string(),
            control: "CancelNo".to_string(),
            type_: "PushButton".to_string(),
            x: 132,
            y: 57,
            width: 56,
            height: 17,
            attributes: 3,
            property: None,
            text: Some("Continue".to_string()),
            control_next: Some("CancelYes".to_string()),
            help: None,
        },
        Control {
            dialog: "CancelDialog".to_string(),
            control: "CancelText".to_string(),
            type_: "Text".to_string(),
            x: 48,
            y: 15,
            width: 194,
            height: 30,
            attributes: 3,
            property: None,
            text: Some("Do you want to abort [ProductName] [Text_action]?".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "CancelDialog".to_string(),
            control: "CancelYes".to_string(),
            type_: "PushButton".to_string(),
            x: 72,
            y: 57,
            width: 56,
            height: 17,
            attributes: 3,
            property: None,
            text: Some("Abort".to_string()),
            control_next: Some("CancelNo".to_string()),
            help: None,
        },
    ]
}
