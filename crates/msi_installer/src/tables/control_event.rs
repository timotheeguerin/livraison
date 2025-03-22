use super::{Entity, RowView, error::MsiDataBaseError};

/// Control Event Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/controlevent-table
#[derive(Debug, Clone)]
pub struct ControlEvent {
    pub dialog: String,
    pub control: String,
    pub event: String,
    pub argument: String,
    pub condition: Option<String>,
    pub ordering: Option<i32>,
}

impl Entity for ControlEvent {
    fn table_name() -> &'static str {
        "ControlEvent"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Dialog_").primary_key().id_string(72),
            msi::Column::build("Control_")
                .primary_key()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Event")
                .primary_key()
                .category(msi::Category::Formatted)
                .string(50),
            msi::Column::build("Argument")
                .primary_key()
                .category(msi::Category::Formatted)
                .string(255),
            msi::Column::build("Condition")
                .primary_key()
                .nullable()
                .category(msi::Category::Condition)
                .string(255),
            msi::Column::build("Ordering")
                .nullable()
                .range(0, 0x7fffffff)
                .int16(),
        ]
    }

    fn from_row(row: &RowView) -> Result<ControlEvent, MsiDataBaseError> {
        Ok(ControlEvent {
            dialog: row.string(0)?,
            control: row.string(1)?,
            event: row.string(2)?,
            argument: row.string(3)?,
            condition: row.opt_string(4)?,
            ordering: row.opt_i32(5)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.dialog.clone()),
            msi::Value::Str(self.control.clone()),
            msi::Value::Str(self.event.clone()),
            msi::Value::Str(self.argument.clone()),
            msi::Value::from_opt_string(&self.condition),
            msi::Value::from_opt_i32(&self.ordering),
        ]
    }
}
