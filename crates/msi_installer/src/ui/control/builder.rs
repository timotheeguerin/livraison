use crate::tables::{Control, ControlEvent, EventMapping};

pub trait ControlBuilder {
    fn id(&self) -> String;
    fn interactive(&self) -> bool;
    fn build(&self, dialog_id: &str) -> Control;
    fn events(&self, _dialog_id: &str) -> Vec<ControlEvent> {
        Vec::new()
    }

    fn event_mappings(&self, _dialog_id: &str) -> Vec<EventMapping> {
        Vec::new()
    }
}
