use crate::{
    tables::{Control, ControlAttributes, ControlType},
    ui::{position::Position, size::Size},
};

use super::ControlBuilder;

pub fn line(id: &str) -> Line {
    Line {
        id: id.to_string(),
        pos: Position::ZERO,
        size: Size::ZERO,
        attributes: ControlAttributes::Visible,
    }
}

#[derive(Debug, Default)]
pub struct Line {
    id: String,
    pos: Position,
    size: Size,
    attributes: ControlAttributes,
}

impl Line {
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
}

impl ControlBuilder for Line {
    fn interactive(&self) -> bool {
        false
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn build(&self, dialog_id: &str) -> Control {
        Control {
            type_: ControlType::Line,
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
}
