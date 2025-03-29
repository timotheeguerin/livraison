use std::vec;

use msi_installer::{
    tables::{Control, ControlAttributes, ControlEvent, ControlType, Dialog, DialogStyle},
    ui::{self, dialog::DialogSize},
};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("ProgressDialog", "[ProductName] Setup")
        .size(DialogSize::classic())
        .modeless()
        .add(
            ui::control::text("Title", "{\\TitleFont}Installing [ProductName]")
                .pos((20, 15))
                .size((330, 15)),
        )
        .add(
            ui::control::text(
                "Text",
                "Please wait while [ProductName] is [Text_done]. This may take several minutes.",
            )
            .pos((20, 65))
            .size((330, 35)),
        )
        .add(ui::control::line("BottomLine").pos((0, 234)).width(374))
        .add(ui::control::line("BannerLine").pos((0, 44)).width(374))
        .add(
            ui::control::text("StatusLabel", "Status:")
                .pos((20, 100))
                .size((50, 10)),
        )
        .add(
            ui::control::progress_bar("ProgressBar")
                .pos((20, 115))
                .width(330),
        )
        .add(
            ui::control::button("Cancel", "Cancel")
                .pos((304, 243))
                .trigger(ui::event::spawn_dialog("CancelDialog")),
        )
        .add(
            ui::control::button("Next", "Next")
                .pos((236, 243))
                .disable(),
        )
        .add(
            ui::control::button("Back", "Back")
                .pos((180, 243))
                .disable(),
        )
}

pub fn create_progress_dialog() -> Dialog {
    Dialog {
        dialog: "ProgressDialog".to_string(),
        h_centering: 50,
        v_centering: 50,
        width: 370,
        height: 270,
        attributes: DialogStyle::Visible,
        title: Some("[ProductName] Setup".to_string()),
        control_first: "Cancel".to_string(),
        control_default: Some("Cancel".to_string()),
        control_cancel: Some("Cancel".to_string()),
    }
}

pub fn create_progress_dialog_controls() -> Vec<Control> {
    vec![
        Control {
            dialog: "ProgressDialog".to_string(),
            control: "Title".to_string(),
            type_: ControlType::Text,
            x: 20,
            y: 15,
            width: 200,
            height: 15,
            attributes: ControlAttributes::NoPrefix
                | ControlAttributes::Transparent
                | ControlAttributes::Visible
                | ControlAttributes::Enabled,
            property: None,
            text: Some("[Text_Doing] [ProductName]".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ProgressDialog".to_string(),
            control: "Cancel".to_string(),
            type_: ControlType::PushButton,
            x: 304,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Cancel".to_string()),
            control_next: Some("Next".to_string()),
            help: None,
        },
        Control {
            dialog: "ProgressDialog".to_string(),
            control: "Text".to_string(),
            type_: ControlType::Text,
            x: 35,
            y: 65,
            width: 300,
            height: 25,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some(
                "Please wait while [ProductName] is [Text_done]. This may take several minutes."
                    .to_string(),
            ),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ProgressDialog".to_string(),
            control: "ActionText".to_string(),
            type_: ControlType::Text,
            x: 70,
            y: 105,
            width: 265,
            height: 15,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: None,
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ProgressDialog".to_string(),
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
            dialog: "ProgressDialog".to_string(),
            control: "Next".to_string(),
            type_: ControlType::PushButton,
            x: 236,
            y: 243,
            width: 56,
            height: 17,
            attributes: ControlAttributes::Visible,
            property: None,
            text: Some("Next".to_string()),
            control_next: Some("Cancel".to_string()),
            help: None,
        },
        Control {
            dialog: "ProgressDialog".to_string(),
            control: "BannerLine".to_string(),
            type_: ControlType::Line,
            x: 0,
            y: 44,
            width: 374,
            height: 0,
            attributes: ControlAttributes::Visible,
            property: None,
            text: None,
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ProgressDialog".to_string(),
            control: "ProgressBar".to_string(),
            type_: ControlType::ProgressBar,
            x: 35,
            y: 125,
            width: 300,
            height: 10,
            attributes: ControlAttributes::Progress95 | ControlAttributes::Visible,
            property: None,
            text: Some("Progress done".to_string()),
            control_next: None,
            help: None,
        },
        Control {
            dialog: "ProgressDialog".to_string(),
            control: "StatusLabel".to_string(),
            type_: ControlType::Text,
            x: 35,
            y: 105,
            width: 35,
            height: 10,
            attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
            property: None,
            text: Some("Status:".to_string()),
            control_next: None,
            help: None,
        },
    ]
}

pub fn create_progress_dialog_control_events() -> Vec<ControlEvent> {
    vec![ControlEvent {
        dialog: "ProgressDialog".to_string(),
        control: "Cancel".to_string(),
        event: "SpawnDialog".to_string(),
        argument: "CancelDialog".to_string(),
        condition: Some("1".to_string()),
        ordering: Some(17),
    }]
}
