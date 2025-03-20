use super::{Entity, RowView, error::MsiDataBaseError};

const TABLE_NAME: &str = "InstallUISequence";

#[derive(Debug, Clone)]
pub struct InstallUISequence {
    pub dialog: String,
    pub condition: Option<String>,
    pub order: i32,
}

impl Entity for InstallUISequence {
    fn table_name() -> &'static str {
        TABLE_NAME
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Action").primary_key().id_string(72),
            msi::Column::build("Condition")
                .nullable()
                .category(msi::Category::Condition)
                .string(255),
            msi::Column::build("Sequence")
                .nullable()
                .range(-4, 0x7fff)
                .int16(),
        ]
    }

    fn from_row(row: &RowView) -> Result<InstallUISequence, MsiDataBaseError> {
        Ok(InstallUISequence {
            dialog: row.string(0)?,
            condition: row.opt_string(1)?,
            order: row.i32(2)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.dialog.clone()),
            msi::Value::from_opt_string(&self.condition),
            msi::Value::Int(self.order),
        ]
    }
}
