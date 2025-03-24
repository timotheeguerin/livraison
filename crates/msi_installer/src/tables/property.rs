use super::{Entity, RowView, error::MsiDataBaseError};

/// Property Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/property-table
#[derive(Debug, Clone, Default)]
pub struct Property {
    pub property: String,
    pub value: String,
}

impl Entity for Property {
    fn table_name() -> &'static str {
        "Property"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Property").primary_key().id_string(72),
            msi::Column::build("Value").localizable().text_string(0),
        ]
    }

    fn from_row(row: &RowView) -> Result<Property, MsiDataBaseError> {
        Ok(Property {
            property: row.string(0)?,
            value: row.string(1)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.property.clone()),
            msi::Value::Str(self.value.clone()),
        ]
    }
}
