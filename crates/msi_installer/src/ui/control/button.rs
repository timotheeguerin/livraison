use crate::{
    tables::{Control, ControlAttributes, ControlEvent, ControlType},
    ui::{event::Event, position::Position, size::Size},
};

use super::ControlBuilder;

pub fn button(id: &str, text: &str) -> Button {
    Button {
        id: id.to_string(),
        text: text.to_string(),
        pos: Position::ZERO,
        size: Size::new(56, 17),
        attributes: ControlAttributes::Visible
            | ControlAttributes::Enabled
            | ControlAttributes::Transparent,
        events: Vec::new(),
    }
}

#[derive(Debug, Default)]
pub struct Button {
    id: String,
    text: String,
    pos: Position,
    size: Size,
    attributes: ControlAttributes,
    events: Vec<Event>,
}

impl Button {
    pub fn pos(mut self, pos: impl Into<Position>) -> Self {
        self.pos = pos.into();
        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    pub fn disable(mut self) -> Self {
        self.attributes.remove(ControlAttributes::Enabled);
        self
    }

    pub fn trigger(mut self, size: Event) -> Self {
        self.events.push(size);
        self
    }
}

impl ControlBuilder for Button {
    fn interactive(&self) -> bool {
        true
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn build(&self, dialog_id: &str) -> Control {
        Control {
            type_: ControlType::PushButton,
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

    fn events(&self, dialog_id: &str) -> Vec<ControlEvent> {
        self.events
            .iter()
            .enumerate()
            .map(|(i, x)| x.as_control_event(dialog_id, &self.id, i as i32))
            .collect()
    }
}
