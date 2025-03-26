use std::io::{Read, Seek, Write};

use msi::Package;
use msi_installer::tables::{Component, ComponentAttributes, Entity, Environment};
use uuid::Uuid;

use crate::{LivraisonError, msi::Context};

/// Config to register an environment variable
pub struct EnvironmentAction {
    pub id: String,

    /// Environment variable name name
    pub name: String,

    pub value: String,

    pub kind: EnvironmentActionKind,
}

pub enum EnvironmentActionKind {
    /// Set the environment variable
    Set,
    ///  Append to the environment variable
    Append,
}

pub fn register_environment_vars<F: Read + Seek + Write>(
    package: &mut Package<F>,
    context: &Context,
    actions: &Vec<EnvironmentAction>,
) -> Result<(), LivraisonError> {
    Environment::create_table(package)?;

    let mut environments: Vec<Environment> = Vec::new();
    let mut components: Vec<Component> = Vec::new();
    for action in actions {
        let component_id = format!("env_{}", action.id);
        let uuid = Uuid::new_v5(&context.upgrade_code, component_id.as_bytes());
        components.push(Component {
            component: component_id.clone(),
            id: Some(uuid),
            directory: "INSTALLDIR".to_string(),
            attributes: ComponentAttributes::Bit64,
            condition: None,
            key_path: Some(component_id.clone()),
        });

        environments.push(Environment {
            environment: action.name.clone(),
            name: get_environment_name(action),
            value: Some(get_environment_value(action)),
            component: component_id.clone(),
        });
    }

    Component::insert(package, &components)?;
    Environment::insert(package, &environments)?;
    Ok(())
}

fn get_environment_name(action: &EnvironmentAction) -> String {
    format!("=-{}", action.name)
}

fn get_environment_value(action: &EnvironmentAction) -> String {
    match &action.kind {
        EnvironmentActionKind::Set => action.value.clone(),
        EnvironmentActionKind::Append => format!("[~];{}", action.value),
    }
}
