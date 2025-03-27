use super::{Entity, RowView, error::MsiDataBaseError};

/// FeatureComponent Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/featurecomponent-table
#[derive(Debug, Clone, Default)]
pub struct FeatureComponent {
    pub feature: String,
    pub component: String,
}

impl Entity for FeatureComponent {
    fn table_name() -> &'static str {
        "FeatureComponent"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Feature_")
                .primary_key()
                .foreign_key("Feature", 1)
                .id_string(38),
            msi::Column::build("Component_")
                .primary_key()
                .foreign_key("Component", 1)
                .id_string(72),
        ]
    }

    fn from_row(row: &RowView) -> Result<FeatureComponent, MsiDataBaseError> {
        Ok(FeatureComponent {
            feature: row.string(0)?,
            component: row.string(1)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.feature.clone()),
            msi::Value::from(self.component.clone()),
        ]
    }
}
