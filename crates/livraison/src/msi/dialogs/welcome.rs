use std::vec;

use msi_installer::{
    tables::{Control, ControlAttributes, ControlEvent, ControlType, Dialog, DialogStyle},
    ui::{self, dialog::DialogSize, event::EndDialogAction},
};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("WelcomeDialog", "[ProductName] Setup")
        .size(DialogSize::classic())
        .add(
            ui::control::text(
                "Title",
                "{\\TitleFont}Welcome to the [ProductName] installer",
            )
            .pos((135, 20))
            .size((220, 60)),
        )
        .add(
            ui::control::text(
                "Description",
                "{\\DefaultFont}This will install [ProductName] on your computer. Click Install to continue or Cancel to exit the installer.",
            )
            .pos((135, 70))
            .size((220, 50)),
        )
        .add(
            ui::control::button(
                "Next",
                "Install",
            )
            .pos(( 236, 243))
            .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button(
                "Cancel",
                "Cancel",
            )
            .pos((304, 243))
            .trigger(ui::event::spawn_dialog("CancelDialog")),
        )
        .add(
            ui::control::button(
                "Back",
                "Back",
            )
            .pos((180, 243))
            .disable()
        )
}

pub fn create_welcome_dialog() -> Dialog {
    Dialog {
        dialog: "WelcomeDialog".to_string(),
        h_centering: 50,
        v_centering: 50,
        width: 370,
        height: 270,
        attributes: DialogStyle::Visible | DialogStyle::Modal,
        title: Some("[ProductName] Setup".to_string()),
        control_first: "Next".to_string(),
        control_default: Some("Next".to_string()),
        control_cancel: Some("Cancel".to_string()),
    }
}

pub fn create_welcome_dialog_controls() -> Vec<Control> {
    vec![
        Control {
            dialog: "WelcomeDialog".to_string(),
            control: "Description".to_string(),
            type_: ControlType::Text,
            x: 135,
            y: 70,
            width: 220,
            height: 50,
            attributes: ControlAttributes::NoPrefix | ControlAttributes::Transparent | ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("{\\DefaultFont}This will install [ProductName] on your computer. Click Install to continue or Cancel to exit the installer.".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "WelcomeDialog".to_string(),
            control: "Title".to_string(),
            type_: ControlType::Text,
            x: 135,
            y: 20,
            width: 220,
            height: 60,
            attributes: ControlAttributes::NoPrefix | ControlAttributes::Transparent | ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("{\\TitleFont}Welcome to the [ProductName] installer".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "WelcomeDialog".to_string(),
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
            dialog: "WelcomeDialog".to_string(),
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
            dialog: "WelcomeDialog".to_string(),
            control: "Back".to_string(),
            type_: ControlType::PushButton,
            x: 180,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible,
            property: None,
            text: Some("&Back".to_string()),
            control_next: Some("Next".to_string()),
            help: None,
        },
        Control {
            dialog: "WelcomeDialog".to_string(),
            control: "Next".to_string(),
            type_: ControlType::PushButton,
            x: 236,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Install".to_string()),
            control_next: Some("Cancel".to_string()),
            help: None,
        },
    ]
}

pub fn create_welcome_dialog_control_events() -> Vec<ControlEvent> {
    vec![
        ControlEvent {
            dialog: "WelcomeDialog".to_string(),
            control: "Cancel".to_string(),
            event: "SpawnDialog".to_string(),
            argument: "CancelDialog".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(0),
        },
        ControlEvent {
            dialog: "WelcomeDialog".to_string(),
            control: "Next".to_string(),
            event: "[Mode]".to_string(),
            argument: "Install".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(1),
        },
        ControlEvent {
            dialog: "WelcomeDialog".to_string(),
            control: "Next".to_string(),
            event: "[Text_action]".to_string(),
            argument: "installation".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(2),
        },
        ControlEvent {
            dialog: "WelcomeDialog".to_string(),
            control: "Next".to_string(),
            event: "[Text_agent]".to_string(),
            argument: "installer".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(3),
        },
        ControlEvent {
            dialog: "WelcomeDialog".to_string(),
            control: "Next".to_string(),
            event: "[Text_Doing]".to_string(),
            argument: "Installing".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(4),
        },
        ControlEvent {
            dialog: "WelcomeDialog".to_string(),
            control: "Next".to_string(),
            event: "[Text_done]".to_string(),
            argument: "installed".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(5),
        },
        ControlEvent {
            dialog: "WelcomeDialog".to_string(),
            control: "Next".to_string(),
            event: "EndDialog".to_string(),
            argument: "Return".to_string(),
            condition: Some("1".to_string()),
            ordering: Some(6),
        },
    ]
}
