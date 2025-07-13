use super::{Entity, RowView, error::MsiDataBaseError};

/// Binary Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/binary-table
#[derive(Debug, Clone, Default)]
pub struct Binary {
    pub binary: String,
}

impl Binary {
    pub fn stream_name(&self) -> String {
        format!("{}.{}", Self::table_name(), self.binary).to_string()
    }
}

impl Entity for Binary {
    fn table_name() -> &'static str {
        "Binary"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Name").primary_key().id_string(72),
            // Binary data despite the doc above is not the actual data but a dummy reference...
            // The data should be retrieve from a stream called `Table.PrimaryKey`
            msi::Column::build("Data").binary(),
        ]
    }

    fn from_row(row: &RowView) -> Result<Binary, MsiDataBaseError> {
        Ok(Binary {
            binary: row.string(0)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.binary.clone()),
            msi::Value::Int(-32767),
        ]
    }
}
