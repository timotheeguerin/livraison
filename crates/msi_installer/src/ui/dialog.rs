use crate::tables::{Dialog, DialogStyle};

use super::{control::ControlBuilder, size::Size};

pub fn new(id: &str, title: &str) -> DialogBuilder {
    DialogBuilder {
        id: id.to_string(),
        title: title.to_string(),
        centering: Centering { h: 50, v: 50 },
        size: Size {
            width: 260,
            height: 100,
        },
        attributes: DialogStyle::Visible | DialogStyle::Modal,
        controls: Vec::new(),
    }
}

pub struct DialogBuilder {
    id: String,
    title: String,
    centering: Centering,
    size: Size,
    attributes: DialogStyle,
    controls: Vec<Box<dyn ControlBuilder>>,
}

impl DialogBuilder {
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    pub fn centering(mut self, h: i32, v: i32) -> Self {
        self.centering = Centering { h, v };
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, control: impl ControlBuilder + 'static) -> Self {
        self.controls.push(Box::new(control));
        self
    }

    pub fn as_dialog(&self) -> Dialog {
        Dialog {
            dialog: self.id.clone(),
            h_centering: self.centering.h,
            v_centering: self.centering.v,
            width: self.size.width,
            height: self.size.height,
            attributes: self.attributes.clone(),
            title: Some(self.title.clone()),
            control_first: "".to_string(),
            control_default: None,
            control_cancel: None,
        }
    }

    pub fn as_controls(&self) -> Vec<crate::tables::Control> {
        self.controls
            .iter()
            .map(|control| control.build(&self.id))
            .collect()
    }
}

pub struct DialogSize;
impl DialogSize {
    /// Classic Wix dialog dimensions
    pub fn minimal() -> Size {
        Size {
            width: 260,
            height: 100,
        }
    }
    /// Classic Wix dialog dimensions
    pub fn classic() -> Size {
        Size {
            width: 370,
            height: 270,
        }
    }
}

struct Centering {
    /** Horizontal centering 0-100 */
    h: i32,
    /** Vertical centering 0-100 */
    v: i32,
}
