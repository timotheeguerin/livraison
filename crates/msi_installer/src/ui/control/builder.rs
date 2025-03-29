use crate::tables::{Control, ControlEvent};

pub trait ControlBuilder {
    fn id(&self) -> String;
    fn interactive(&self) -> bool;
    fn build(&self, dialog_id: &str) -> Control;
    fn events(&self, _dialog_id: &str) -> Vec<ControlEvent> {
        Vec::new()
    }
}
