use super::{Entity, RowView, error::MsiDataBaseError};

const TABLE_NAME: &str = "Dialog";

#[derive(Debug, Clone)]
pub struct Dialog {
    pub dialog: String,
    pub h_centering: i16,
    pub v_centering: i16,
    pub width: i16,
    pub height: i16,
    pub attributes: i32,
    pub title: Option<String>,
    pub control_first: String,
    pub control_default: Option<String>,
    pub control_cancel: Option<String>,
}

impl Entity for Dialog {
    fn table_name() -> &'static str {
        TABLE_NAME
    }

    fn from_row(row: &RowView) -> Result<Dialog, MsiDataBaseError> {
        Ok(Dialog {
            dialog: row.string(0)?,
            h_centering: row.i16(1)?,
            v_centering: row.i16(2)?,
            width: row.i16(3)?,
            height: row.i16(4)?,
            attributes: row.i32(5)?,
            title: row.opt_string(6)?,
            control_first: row.string(7)?,
            control_default: row.opt_string(8)?,
            control_cancel: row.opt_string(9)?,
        })
    }
}
