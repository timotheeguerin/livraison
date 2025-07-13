use super::{Entity, RowView, error::MsiDataBaseError};

/// Control Event Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/eventmapping-table
#[derive(Debug, Clone)]
pub struct EventMapping {
    pub dialog: String,
    pub control: String,
    pub event: String,
    pub attribute: String,
}

impl Entity for EventMapping {
    fn table_name() -> &'static str {
        "EventMapping"
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
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Attribute")
                .category(msi::Category::Identifier)
                .string(50),
        ]
    }

    fn from_row(row: &RowView) -> Result<EventMapping, MsiDataBaseError> {
        Ok(EventMapping {
            dialog: row.string(0)?,
            control: row.string(1)?,
            event: row.string(2)?,
            attribute: row.string(3)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.dialog.clone()),
            msi::Value::Str(self.control.clone()),
            msi::Value::Str(self.event.clone()),
            msi::Value::Str(self.attribute.clone()),
        ]
    }
}
