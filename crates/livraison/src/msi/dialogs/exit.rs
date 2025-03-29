use msi_installer::{
    tables::{Control, ControlAttributes, ControlEvent, ControlType, Dialog, DialogStyle},
    ui::{self, dialog::DialogSize, event::EndDialogAction},
};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("ExitDialog", "[ProductName] Setup")
        .size(DialogSize::classic())
        .add(
            ui::control::text("Title", "{\\TitleFont}[ProductName] [Text_action] complete")
                .pos((135, 20))
                .size((220, 60)),
        )
        .add(
            ui::control::text(
                "Description",
                "Click the Finish button to exit the [Text_agent].",
            )
            .pos((135, 70))
            .size((220, 20)),
        )
        .add(ui::control::line("BottomLine").pos((0, 234)).width(374))
        .add(
            ui::control::button("Finish", "Finish")
                .pos((236, 243))
                .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button("Back", "Back")
                .pos((180, 243))
                .disable(),
        )
        .add(
            ui::control::button("Cancel", "Cancel")
                .pos((304, 243))
                .disable(),
        )
}

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
            type_: ControlType::Text,
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
            type_: ControlType::Text,
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
            type_: ControlType::PushButton,
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
            type_: ControlType::PushButton,
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
            dialog: "ExitDialog".to_string(),
            control: "Finish".to_string(),
            type_: ControlType::PushButton,
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
        control: "Finish".to_string(),
        event: "EndDialog".to_string(),
        argument: "Return".to_string(),
        condition: Some("1".to_string()),
        ordering: Some(999),
    }]
}
