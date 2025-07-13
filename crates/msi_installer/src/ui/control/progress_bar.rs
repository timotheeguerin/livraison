use crate::{
    tables::{Control, ControlAttributes, ControlType, EventMapping},
    ui::{position::Position, size::Size},
};

use super::ControlBuilder;

pub fn progress_bar(id: &str) -> ProgressBar {
    ProgressBar {
        id: id.to_string(),
        pos: Position::ZERO,
        size: Size {
            width: 0,
            height: 10,
        },
        attributes: ControlAttributes::Visible | ControlAttributes::Progress95,
        listener: Some("SetProgress".to_string()),
    }
}

#[derive(Debug, Default)]
pub struct ProgressBar {
    id: String,
    pos: Position,
    size: Size,
    attributes: ControlAttributes,
    listener: Option<String>,
}

impl ProgressBar {
    pub fn pos(mut self, pos: impl Into<Position>) -> Self {
        self.pos = pos.into();
        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
    pub fn width(mut self, size: i32) -> Self {
        self.size = Size {
            width: size,
            height: self.size.height,
        };
        self
    }

    /// Set the text when this event is triggered with the event argument.
    pub fn on_event(mut self, event: &str) -> Self {
        self.listener = Some(event.to_string());
        self
    }
}

impl ControlBuilder for ProgressBar {
    fn interactive(&self) -> bool {
        false
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn build(&self, dialog_id: &str) -> Control {
        Control {
            type_: ControlType::ProgressBar,
            dialog: dialog_id.to_string(),
            control: self.id.clone(),
            x: self.pos.x,
            y: self.pos.y,
            width: self.size.width,
            height: self.size.height,
            attributes: self.attributes.clone(),
            text: None,
            property: None,
            control_next: None,
            help: None,
        }
    }

    fn event_mappings(&self, dialog_id: &str) -> Vec<EventMapping> {
        if let Some(event) = &self.listener {
            vec![EventMapping {
                dialog: dialog_id.to_string(),
                control: self.id.clone(),
                event: event.clone(),
                attribute: "Progress".to_string(),
            }]
        } else {
            Vec::new()
        }
    }
}
