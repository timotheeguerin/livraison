use std::{
    collections::HashMap,
    io::{Read, Seek, Write},
};

use crate::tables::{
    Control, ControlEvent, Dialog, Entity, EventMapping, InstallUISequence, StyleAttributes,
    TextStyle,
};

use super::dialog::{self, DialogBuilder};

pub fn new() -> UiBuilder {
    UiBuilder {
        title: "[ProductName] Setup".to_string(),
        dialogs: HashMap::new(),
    }
}

pub struct UiBuilder {
    title: String,
    dialogs: HashMap<String, DialogBuilder>,
}

impl UiBuilder {
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn new_dialog<F: Fn(DialogBuilder) -> DialogBuilder>(mut self, id: &str, cb: F) -> Self {
        let dialog = dialog::new(id, &self.title);
        self.dialogs.insert(id.to_string(), cb(dialog));
        self
    }

    pub fn insert<F: Read + Write + Seek>(
        &self,
        package: &mut msi::Package<F>,
    ) -> Result<(), std::io::Error> {
        self.insert_fonts(package)?;
        self.insert_ui_sequence(package)?;
        Dialog::create_table(package)?;
        Control::create_table(package)?;
        ControlEvent::create_table(package)?;
        EventMapping::create_table(package)?;

        Dialog::insert(
            package,
            &self
                .dialogs
                .values()
                .map(|dialog| dialog.dialog())
                .collect::<Vec<Dialog>>(),
        )?;

        Control::insert(
            package,
            &self
                .dialogs
                .values()
                .flat_map(|dialog| dialog.controls())
                .collect::<Vec<Control>>(),
        )?;
        ControlEvent::insert(
            package,
            &self
                .dialogs
                .values()
                .flat_map(|dialog| dialog.events())
                .collect::<Vec<ControlEvent>>(),
        )?;
        EventMapping::insert(
            package,
            &self
                .dialogs
                .values()
                .flat_map(|dialog| dialog.event_mappings())
                .collect::<Vec<EventMapping>>(),
        )?;
        Ok(())
    }

    fn insert_fonts(
        &self,
        package: &mut msi::Package<impl Read + Write + Seek>,
    ) -> Result<(), std::io::Error> {
        TextStyle::create_table(package)?;
        TextStyle::insert(
            package,
            &[
                TextStyle {
                    text_style: "DefaultFont".to_string(),
                    face_name: "Segoe UI".to_string(),
                    size: 8,
                    color: 0,
                    attributes: None,
                },
                TextStyle {
                    text_style: "BoldFont".to_string(),
                    face_name: "Segoe UI".to_string(),
                    size: 12,
                    color: 0,
                    attributes: Some(StyleAttributes::Bold),
                },
                TextStyle {
                    text_style: "TitleFont".to_string(),
                    face_name: "Segoe UI".to_string(),
                    size: 9,
                    color: 0,
                    attributes: Some(StyleAttributes::Bold),
                },
            ],
        )?;
        Ok(())
    }

    fn insert_ui_sequence(
        &self,
        package: &mut msi::Package<impl Read + Write + Seek>,
    ) -> Result<(), std::io::Error> {
        InstallUISequence::create_table(package)?;
        let mut seq = Vec::new();

        fn add_ui_dlg(
            id: &str,
            condition: Option<String>,
            order: i32,
            seq: &mut Vec<InstallUISequence>,
            self_dialogs: &HashMap<String, DialogBuilder>,
        ) {
            if self_dialogs.contains_key(id) {
                seq.push(InstallUISequence {
                    dialog: id.to_string(),
                    condition,
                    order,
                });
            }
        }

        add_ui_dlg("FatalErrorDlg", None, -3, &mut seq, &self.dialogs);
        add_ui_dlg("UserErrorDlg", None, -2, &mut seq, &self.dialogs);
        add_ui_dlg("ExitDlg", None, -1, &mut seq, &self.dialogs);
        // add_known_dlg("LaunchConditions", None, 100);
        // add_known_dlg("FindRelatedProducts", None, 200);
        // add_known_dlg("AppSearch", None, 400);
        // add_known_dlg("CCPSearch", Some("NOT Installed".to_string()), 500);
        // add_known_dlg("RMCCPSearch", Some("NOT Installed".to_string()), 600);
        seq.push(InstallUISequence::new("CostInitialize", None, 800));
        seq.push(InstallUISequence::new("FileCost", None, 900));
        seq.push(InstallUISequence::new("CostFinalize", None, 1000));
        // add_known_dlg("MigrateFeatureStates", None, 1200);
        add_ui_dlg(
            "WelcomeDlg",
            Some("NOT Installed".to_string()),
            1230,
            &mut seq,
            &self.dialogs,
        );
        add_ui_dlg(
            "RemoveDlg",
            Some("Installed".to_string()),
            1240,
            &mut seq,
            &self.dialogs,
        );
        add_ui_dlg("ProgressDlg", None, 1280, &mut seq, &self.dialogs);
        seq.push(InstallUISequence::new("ExecuteAction", None, 1300));

        InstallUISequence::insert(package, &seq)?;
        Ok(())
    }
}
