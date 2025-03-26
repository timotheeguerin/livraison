use strum_macros::FromRepr;

use super::{Entity, RowView, error::MsiDataBaseError};

/// Registry Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/registry-table
#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub registry: String,
    pub root: RegistryRoot,
    pub key: String,
    pub name: String,
    pub value: String,
    pub component: String,
}

impl Entity for Registry {
    fn table_name() -> &'static str {
        "Registry"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Registry")
                .primary_key()
                .category(msi::Category::Identifier)
                .id_string(72),
            msi::Column::build("Root").int16(),
            msi::Column::build("Key")
                .category(msi::Category::RegPath)
                .localizable()
                .string(255),
            msi::Column::build("Name")
                .localizable()
                .category(msi::Category::Formatted)
                .nullable()
                .string(255),
            msi::Column::build("Value")
                .localizable()
                .category(msi::Category::Formatted)
                .nullable()
                .string(0),
            msi::Column::build("Component_")
                .foreign_key("Component", 1)
                .category(msi::Category::Identifier)
                .string(72),
        ]
    }

    fn from_row(row: &RowView) -> Result<Registry, MsiDataBaseError> {
        Ok(Registry {
            registry: row.string(0)?,
            root: match RegistryRoot::from_repr(row.i32(1)?) {
                Some(root) => root,
                None => return Err(MsiDataBaseError::InvalidRegistryRoot(row.i32(1)?)),
            },
            key: row.string(2)?,
            name: row.string(3)?,
            value: row.string(4)?,
            component: row.string(5)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.registry.clone()),
            msi::Value::Int(self.root.clone() as i32),
            msi::Value::Str(self.key.clone()),
            msi::Value::Str(self.name.clone()),
            msi::Value::Str(self.value.clone()),
            msi::Value::Str(self.component.clone()),
        ]
    }
}

/// The predefined root key for the registry value.
/// Enter a value of -1 in this field to make the root key dependent on the type of installation.
/// Enter one of the other values in the following table to force the registry value to be written under a particular root key.
#[derive(Debug, Clone, FromRepr)]
#[repr(i32)]
#[derive(Default)]
pub enum RegistryRoot {
    /// If this is a per-user installation, the registry value is written under HKEY_CURRENT_USER.
    /// If this is a per-machine installation, the registry value is written under HKEY_LOCAL_MACHINE.
    /// Note that a per-machine installation is specified by setting the ALLUSERS property to 1.
    #[default]
    Auto = -1,

    /// HKEY_CLASSES_ROOTThe installer writes or removes the value from the HKCU\Software\Classes hive during installation in the per-user installation context.
    /// The installer writes or removes the value from the HKLM\Software\Classes hive during per-machine installations.
    ClassesRoot = 0,

    /// HKEY_CURRENT_USER
    CurrentUser = 1,
    /// HKEY_LOCAL_MACHINE
    LocalMachine = 2,

    /// HKEY_USERS
    Users = 3,
}
