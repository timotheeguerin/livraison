use super::{Entity, RowView, error::MsiDataBaseError};

const TABLE_NAME: &str = "Dialog";

pub struct Dialog {
    pub dialog: String,
    pub h_centering: i16,
    pub v_centering: i16,
    pub width: i16,
    pub height: i16,
    pub attributes: i32,
    pub title: String,
    pub control_first: String,
    pub control_default: String,
    pub control_cancel: String,
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
            title: row.string(6)?,
            control_first: row.string(7)?,
            control_default: row.string(8)?,
            control_cancel: row.string(9)?,
        })
    }
}
