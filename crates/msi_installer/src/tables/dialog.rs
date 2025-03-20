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

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Dialog").primary_key().id_string(72),
            msi::Column::build("HCentering").range(0, 100).int16(),
            msi::Column::build("VCentering").range(0, 100).int16(),
            msi::Column::build("Width").range(0, 0x7fff).int16(),
            msi::Column::build("Height").range(0, 0x7fff).int16(),
            msi::Column::build("Attributes")
                .nullable()
                .range(-4, 0x7fffffff)
                .int32(),
            msi::Column::build("Title")
                .nullable()
                .category(msi::Category::Formatted)
                .string(128),
            msi::Column::build("Control_First")
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Control_Default")
                .nullable()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Control_Cancel")
                .nullable()
                .category(msi::Category::Identifier)
                .string(50),
        ]
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
