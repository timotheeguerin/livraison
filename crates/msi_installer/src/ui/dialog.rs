use crate::tables::{Control, ControlEvent, Dialog, DialogStyle};

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
        attributes: DialogStyle::Visible | DialogStyle::Modal | DialogStyle::Minimize,
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

    /// Disable Modal mode https://learn.microsoft.com/en-us/windows/win32/msi/modal-dialog-style-bit
    pub fn modeless(mut self) -> Self {
        self.attributes.remove(DialogStyle::Modal);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, control: impl ControlBuilder + 'static) -> Self {
        self.controls.push(Box::new(control));
        self
    }

    pub fn dialog(&self) -> Dialog {
        let first = self
            .controls
            .iter()
            .find(|x| x.interactive())
            .map(|x| x.id());

        let cancel = self
            .controls
            .iter()
            .find(|x| x.id() == "Cancel")
            .map(|x| x.id());
        Dialog {
            dialog: self.id.clone(),
            h_centering: self.centering.h,
            v_centering: self.centering.v,
            width: self.size.width,
            height: self.size.height,
            attributes: self.attributes.clone(),
            title: Some(self.title.clone()),
            control_first: first
                .clone()
                .expect("Dialog must have at least one interactive control"),
            control_default: first,
            control_cancel: cancel,
        }
    }

    pub fn controls(&self) -> Vec<Control> {
        let mut next = self
            .controls
            .iter()
            .find(|x| x.interactive())
            .map(|x| x.id());

        let mut controls: Vec<Control> = Vec::new();

        for control in self.controls.iter().rev() {
            let mut item = control.build(&self.id);
            if control.interactive() {
                item.control_next = next;
                next = Some(item.control.clone());
            }
            controls.push(item)
        }

        controls.reverse();
        controls
    }

    pub fn events(&self) -> Vec<ControlEvent> {
        let mut events: Vec<ControlEvent> = Vec::new();
        for control in self.controls.iter().rev() {
            events.extend(control.events(&self.id));
        }

        events
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
