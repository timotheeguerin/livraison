use bitflags::bitflags;
use uuid::Uuid;

use super::{Entity, RowView, error::MsiDataBaseError};

/// Component Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/component-table
#[derive(Debug, Clone, Default)]
pub struct Component {
    pub component: String,
    pub id: Option<Uuid>,
    pub directory: String,
    pub attributes: ComponentAttributes,
    pub condition: Option<String>,
    pub key_path: Option<String>,
}

impl Entity for Component {
    fn table_name() -> &'static str {
        "Component"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Component").primary_key().id_string(72),
            msi::Column::build("ComponentId")
                .nullable()
                .category(msi::Category::Guid)
                .string(38),
            msi::Column::build("Directory_")
                .nullable()
                .foreign_key("Directory", 1)
                .id_string(72),
            msi::Column::build("Attributes").int16(),
            msi::Column::build("Condition")
                .nullable()
                .category(msi::Category::Condition)
                .string(255),
            msi::Column::build("KeyPath").nullable().id_string(72),
        ]
    }

    fn from_row(row: &RowView) -> Result<Component, MsiDataBaseError> {
        Ok(Component {
            component: row.string(0)?,
            id: row.opt_uuid(1)?,
            directory: row.string(2)?,
            attributes: ComponentAttributes::from_bits_retain(row.i32(3)?),
            condition: row.opt_string(4)?,
            key_path: row.opt_string(5)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.component.clone()),
            msi::Value::from(self.id),
            msi::Value::Str(self.directory.clone()),
            msi::Value::Int(self.attributes.bits()),
            msi::Value::from_opt_string(&self.condition),
            msi::Value::from_opt_string(&self.key_path),
        ]
    }
}

bitflags! {
    /// Component Attributes
    /// https://learn.microsoft.com/en-us/windows/win32/msi/component-table#Attributes
    #[derive(Debug, Clone, Default)]

    pub struct ComponentAttributes: i32 {
        /// Component can only be run from source.
        /// Set this bit for all components belonging to a feature to prevent the feature from being run-from-my-computer.
        const SourceOnly = 1;
        /// Component can run locally or from source.
        const Optional = 2;
        /// If this bit is set, the value in the KeyPath column is used as a key into the Registry table.
        const RegistryKeyPath = 4;
        const SharedDllRefCount = 8;
        const Permanent = 16;
        const OdbcDataSource = 32;
        const Transitive = 64;
        const NeverOverwrite = 128;
        const Bit64 = 256;
        const DisableRegistryReflection = 512;
        const UninstallOnSupersedence = 1024;
        const Shared = 2048;
    }
}
