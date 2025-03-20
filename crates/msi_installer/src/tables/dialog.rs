use super::{Entity, RowView, error::MsiDataBaseError};

const TABLE_NAME: &str = "Dialog";

#[derive(Debug, Clone)]
pub struct Dialog {
    pub dialog: String,
    pub h_centering: i32,
    pub v_centering: i32,
    pub width: i32,
    pub height: i32,
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
            h_centering: row.i32(1)?,
            v_centering: row.i32(2)?,
            width: row.i32(3)?,
            height: row.i32(4)?,
            attributes: row.i32(5)?,
            title: row.opt_string(6)?,
            control_first: row.string(7)?,
            control_default: row.opt_string(8)?,
            control_cancel: row.opt_string(9)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.dialog.clone()),
            msi::Value::Int(self.h_centering),
            msi::Value::Int(self.v_centering),
            msi::Value::Int(self.width),
            msi::Value::Int(self.height),
            msi::Value::Int(self.attributes),
            msi::Value::from_opt_string(&self.title),
            msi::Value::Str(self.control_first.clone()),
            msi::Value::from_opt_string(&self.control_default),
            msi::Value::from_opt_string(&self.control_cancel),
        ]
    }
}
