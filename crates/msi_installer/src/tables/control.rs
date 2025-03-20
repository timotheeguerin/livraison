use super::{Entity, RowView, error::MsiDataBaseError};

/// Control Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/control-table
#[derive(Debug, Clone)]
pub struct Control {
    pub dialog: String,
    pub control: String,
    pub type_: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub attributes: i32,
    pub property: Option<String>,
    pub text: Option<String>,
    pub control_next: Option<String>,
    pub help: Option<String>,
}

impl Entity for Control {
    fn table_name() -> &'static str {
        "Control"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Dialog_").id_string(72),
            msi::Column::build("Control")
                .primary_key()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Type")
                .category(msi::Category::Identifier)
                .string(20),
            msi::Column::build("X").range(0, 0x7fff).int16(),
            msi::Column::build("Y").range(0, 0x7fff).int16(),
            msi::Column::build("Width").range(0, 0x7fff).int16(),
            msi::Column::build("Height").range(0, 0x7fff).int16(),
            msi::Column::build("Attributes")
                .nullable()
                .range(-4, 0x7fffffff)
                .int32(),
            msi::Column::build("Property")
                .nullable()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Text")
                .nullable()
                .category(msi::Category::Formatted)
                .string(0),
            msi::Column::build("Control_Next")
                .nullable()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Help")
                .nullable()
                .category(msi::Category::Text)
                .string(50),
        ]
    }

    fn from_row(row: &RowView) -> Result<Control, MsiDataBaseError> {
        Ok(Control {
            dialog: row.string(0)?,
            control: row.string(1)?,
            type_: row.string(2)?,
            x: row.i32(3)?,
            y: row.i32(4)?,
            width: row.i32(5)?,
            height: row.i32(6)?,
            attributes: row.i32(7)?,
            property: row.opt_string(8)?,
            text: row.opt_string(9)?,
            control_next: row.opt_string(10)?,
            help: row.opt_string(11)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.dialog.clone()),
            msi::Value::Str(self.control.clone()),
            msi::Value::Str(self.type_.clone()),
            msi::Value::Int(self.x),
            msi::Value::Int(self.y),
            msi::Value::Int(self.width),
            msi::Value::Int(self.height),
            msi::Value::Int(self.attributes),
            msi::Value::from_opt_string(&self.property),
            msi::Value::from_opt_string(&self.text),
            msi::Value::from_opt_string(&self.control_next),
            msi::Value::from_opt_string(&self.help),
        ]
    }
}
