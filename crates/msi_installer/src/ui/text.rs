use crate::tables::{Control, ControlAttributes, ControlType};

use super::{control::ControlBuilder, position::Position, size::Size};

pub fn text(id: &str, text: &str) -> Text {
    Text {
        id: id.to_string(),
        text: text.to_string(),
        pos: Position::ZERO,
        size: Size::ZERO,
        attributes: ControlAttributes::Visible,
    }
}

#[derive(Debug, Default)]
pub struct Text {
    id: String,
    text: String,
    pos: Position,
    size: Size,
    attributes: ControlAttributes,
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
}

impl ControlBuilder for Text {
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
            text: Some(self.text.clone()),
            property: None,
            control_next: None,
            help: None,
        }
    }
}
