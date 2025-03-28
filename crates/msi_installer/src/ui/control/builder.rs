use crate::tables::Control;

pub trait ControlBuilder {
    fn id(&self) -> String;
    fn interactive(&self) -> bool;
    fn build(&self, dialog_id: &str) -> Control;
}
