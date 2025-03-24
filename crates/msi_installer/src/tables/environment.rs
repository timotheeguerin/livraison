use super::{Entity, RowView, error::MsiDataBaseError};

/// Environment Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/environment-table
#[derive(Debug, Clone, Default)]
pub struct Environment {
    pub environment: String,
    pub name: String,
    pub value: Option<String>,
    pub component: String,
}

impl Entity for Environment {
    fn table_name() -> &'static str {
        "Environment"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Environment")
                .primary_key()
                .id_string(72),
            msi::Column::build("Name").localizable().string(255),
            msi::Column::build("Value").localizable().string(255),
            msi::Column::build("Component_")
                .foreign_key("Component", 1)
                .id_string(72),
        ]
    }

    fn from_row(row: &RowView) -> Result<Environment, MsiDataBaseError> {
        Ok(Environment {
            environment: row.string(0)?,
            name: row.string(1)?,
            value: row.opt_string(2)?,
            component: row.string(3)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.environment.clone()),
            msi::Value::Str(self.name.clone()),
            msi::Value::from_opt_string(&self.value),
            msi::Value::Str(self.component.clone()),
        ]
    }
}
