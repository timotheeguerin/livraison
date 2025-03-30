use std::io::{Read, Seek, Write};

use crate::tables::{
    Control, ControlEvent, Dialog, Entity, EventMapping, StyleAttributes, TextStyle,
};

use super::dialog::{self, DialogBuilder};

pub fn new() -> UiBuilder {
    UiBuilder {
        title: "[ProductName] Setup".to_string(),
        dialogs: Vec::new(),
    }
}

pub struct UiBuilder {
    title: String,
    dialogs: Vec<DialogBuilder>,
}

impl UiBuilder {
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn new_dialog<F: Fn(DialogBuilder) -> DialogBuilder>(mut self, id: &str, cb: F) -> Self {
        let dialog = dialog::new(id, &self.title);
        self.dialogs.push(cb(dialog));
        self
    }

    pub fn insert<F: Read + Write + Seek>(
        &self,
        package: &mut msi::Package<F>,
    ) -> Result<(), std::io::Error> {
        self.insert_fonts(package)?;
        Dialog::create_table(package)?;
        Control::create_table(package)?;
        ControlEvent::create_table(package)?;
        EventMapping::create_table(package)?;

        Dialog::insert(
            package,
            &self
                .dialogs
                .iter()
                .map(|dialog| dialog.dialog())
                .collect::<Vec<Dialog>>(),
        )?;

        Control::insert(
            package,
            &self
                .dialogs
                .iter()
                .flat_map(|dialog| dialog.controls())
                .collect::<Vec<Control>>(),
        )?;
        ControlEvent::insert(
            package,
            &self
                .dialogs
                .iter()
                .flat_map(|dialog| dialog.events())
                .collect::<Vec<ControlEvent>>(),
        )?;
        EventMapping::insert(
            package,
            &self
                .dialogs
                .iter()
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
}
