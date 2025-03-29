use crate::{
    tables::{Control, ControlAttributes, ControlType, EventMapping},
    ui::{position::Position, size::Size},
};

use super::ControlBuilder;

pub fn dyn_text(id: &str, event: &str) -> Text {
    Text {
        id: id.to_string(),
        text: None,
        pos: Position::ZERO,
        size: Size::ZERO,
        attributes: ControlAttributes::Visible | ControlAttributes::Enabled,
        listener: Some(event.to_string()),
    }
}
pub fn text(id: &str, text: &str) -> Text {
    Text {
        id: id.to_string(),
        text: Some(text.to_string()),
        pos: Position::ZERO,
        size: Size::ZERO,
        attributes: ControlAttributes::Visible,
        listener: None,
    }
}

#[derive(Debug, Default)]
pub struct Text {
    id: String,
    text: Option<String>,
    pos: Position,
    size: Size,
    attributes: ControlAttributes,
    listener: Option<String>,
}

impl Text {
    pub fn pos(mut self, pos: impl Into<Position>) -> Self {
        self.pos = pos.into();
        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Enable the text control for event mapping.
    pub fn enable(mut self) -> Self {
        self.attributes |= ControlAttributes::Enabled;
        self
    }

    /// Set the text when this event is triggered with the event argument.
    pub fn on_event(mut self, event: &str) -> Self {
        self.listener = Some(event.to_string());
        self
    }
}

impl ControlBuilder for Text {
    fn interactive(&self) -> bool {
        false
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn build(&self, dialog_id: &str) -> Control {
        Control {
            type_: ControlType::Text,
            dialog: dialog_id.to_string(),
            control: self.id.clone(),
            x: self.pos.x,
            y: self.pos.y,
            width: self.size.width,
            height: self.size.height,
            attributes: self.attributes.clone(),
            text: self.text.clone(),
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
                attribute: "Text".to_string(),
            }]
        } else {
            Vec::new()
        }
    }
}
