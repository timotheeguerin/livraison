use std::vec;

use msi_installer::tables::{
    Control, ControlAttributes, ControlEvent, ControlType, Dialog, DialogStyle,
};

pub fn create_remove_dialog() -> Dialog {
    Dialog {
        dialog: "RemoveDialog".to_string(),
        h_centering: 50,
        v_centering: 50,
        width: 370,
        height: 270,
        attributes: DialogStyle::Visible | DialogStyle::Modal,
        title: Some("[ProductName] Setup".to_string()),
        control_first: "Remove".to_string(),
        control_default: Some("Remove".to_string()),
        control_cancel: Some("Remove".to_string()),
    }
}

pub fn create_remove_dialog_controls() -> Vec<Control> {
    vec![
        Control {
            dialog: "RemoveDialog".to_string(),
            control: "Description".to_string(),
            type_: ControlType::Text,
            x: 135,
            y: 70,
            width: 220,
            height: 50,
            attributes: ControlAttributes::NoPrefix | ControlAttributes::Transparent | ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("This will remove [ProductName] from your computer. Click Remove to continue or Cancel to exit the uninstaller.".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "RemoveDialog".to_string(),
            control: "Title".to_string(),
            type_: ControlType::Text,
            x: 135,
            y: 20,
            width: 220,
            height: 60,
            attributes: ControlAttributes::NoPrefix | ControlAttributes::Transparent | ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("{\\TitleFont}Uninstall [ProductName]".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "RemoveDialog".to_string(),
            control: "Cancel".to_string(),
            type_: ControlType::PushButton,
            x: 304,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Cancel".to_string()),
            control_next: Some("Back".to_string()),
            help: None,
        },
        Control {
            dialog: "RemoveDialog".to_string(),
            control: "Back".to_string(),
            type_: ControlType::PushButton,
            x: 180,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible,
            property: None,
            text: Some("Back".to_string()),
            control_next: Some("Remove".to_string()),
            help: None,
        },
        Control {
            dialog: "RemoveDialog".to_string(),
            control: "BottomLine".to_string(),
            type_: ControlType::Line,
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
            dialog: "RemoveDialog".to_string(),
            control: "Remove".to_string(),
            type_: ControlType::PushButton,
            x: 236,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Remove".to_string()),
            control_next: Some("Cancel".to_string()),
            help: None,
        },
    ]
}

pub fn create_remove_dialog_control_events() -> Vec<ControlEvent> {
    vec![
        ControlEvent {
            dialog: "RemoveDialog".to_string(),
            control: "Cancel".to_string(),
            event: "[Text_action]".to_string(),
            argument: "removal".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(7),
        },
        ControlEvent {
            dialog: "RemoveDialog".to_string(),
            control: "Cancel".to_string(),
            event: "SpawnDialog".to_string(),
            argument: "CancelDialog".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(8),
        },
        ControlEvent {
            dialog: "RemoveDialog".to_string(),
            control: "Remove".to_string(),
            event: "Remove".to_string(),
            argument: "All".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(14),
        },
        ControlEvent {
            dialog: "RemoveDialog".to_string(),
            control: "Remove".to_string(),
            event: "EndDialog".to_string(),
            argument: "Return".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(14),
        },
    ]
}
