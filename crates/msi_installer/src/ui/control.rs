use crate::tables::Control;

pub trait ControlBuilder {
    fn build(&self, dialog_id: &str) -> Control;
}
