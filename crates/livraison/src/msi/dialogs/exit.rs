use msi_installer::tables::{Control, ControlAttributes, ControlEvent, Dialog, DialogStyle};

pub fn create_exit_dialog() -> Dialog {
    Dialog {
        dialog: "ExitDialog".to_string(),
        h_centering: 50,
        v_centering: 50,
        width: 370,
        height: 270,
        attributes: DialogStyle::Visible | DialogStyle::Modal,
        title: Some("[ProductName] Setup".to_string()),
        control_first: "Finish".to_string(),
        control_default: Some("Finish".to_string()),
        control_cancel: Some("Finish".to_string()),
    }
}

pub fn create_exit_dialog_controls() -> Vec<Control> {
    vec![
        Control {
            dialog: "ExitDialog".to_string(),
            control: "Description".to_string(),
            type_: "Text".to_string(),
            x: 135,
            y: 70,
            width: 220,
            height: 20,
            attributes: ControlAttributes::NoPrefix
                | ControlAttributes::Transparent
                | ControlAttributes::Visible
                | ControlAttributes::Enabled,
            property: None,
            text: Some("Click the Finish button to exit the [Text_agent].".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ExitDialog".to_string(),
            control: "Title".to_string(),
            type_: "Text".to_string(),
            x: 135,
            y: 20,
            width: 220,
            height: 60,
            attributes: ControlAttributes::NoPrefix
                | ControlAttributes::Transparent
                | ControlAttributes::Visible
                | ControlAttributes::Enabled,
            property: None,
            text: Some("{\\TitleFont}[ProductName] [Text_action] complete".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ExitDialog".to_string(),
            control: "Cancel".to_string(),
            type_: "PushButton".to_string(),
            x: 304,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible,
            property: None,
            text: Some("Cancel".to_string()),
            control_next: Some("Back".to_string()),
            help: None,
        },
        Control {
            dialog: "ExitDialog".to_string(),
            control: "Back".to_string(),
            type_: "PushButton".to_string(),
            x: 180,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible,
            property: None,
            text: Some("Back".to_string()),
            control_next: Some("Finish".to_string()),
            help: None,
        },
        Control {
            dialog: "ExitDialog".to_string(),
            control: "BottomLine".to_string(),
            type_: "Line".to_string(),
            x: 0,
            y: 234,
            width: 374,
            height: 0,
            attributes: ControlAttributes::Visible,
            property: None,
            text: None,
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ExitDialog".to_string(),
            control: "Finish".to_string(),
            type_: "PushButton".to_string(),
            x: 236,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Finish".to_string()),
            control_next: Some("Cancel".to_string()),
            help: None,
        },
    ]
}

pub fn create_exit_dialog_control_events() -> Vec<ControlEvent> {
    vec![ControlEvent {
        dialog: "ExitDialog".to_string(),
        control: "ExitFinish".to_string(),
        event: "EndDialog".to_string(),
        argument: "Return".to_string(),
        condition: Some("1".to_string()),
        ordering: Some(18),
    }]
}
