use std::io::{Read, Seek, Write};

use msi::Package;
use msi_installer::tables::{
    Component, ComponentAttributes, Entity, Environment, FeatureComponents, Registry,
};
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

#[allow(dead_code)]
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
    Registry::create_table(package)?;

    let mut environments: Vec<Environment> = Vec::new();
    let mut components: Vec<Component> = Vec::new();
    let mut feature_components: Vec<FeatureComponents> = Vec::new();
    // let mut registry_items: Vec<Registry> = Vec::new();
    for action in actions {
        let component_id = format!("env_{}", action.id.to_lowercase());
        // let component_id = "reg384648C2D0DE5577014AD3CB66D5A086".to_string();
        let uuid = Uuid::new_v5(&context.upgrade_code, component_id.as_bytes());
        components.push(Component {
            component: component_id.clone(),
            id: Some(uuid),
            directory: "INSTALLDIR".to_string(),
            attributes: ComponentAttributes::Bit64, // | ComponentAttributes::RegistryKeyPath,
            condition: None,
            key_path: None,
            // key_path: Some(component_id.clone()),
        });

        environments.push(Environment {
            environment: action.name.clone(),
            name: get_environment_name(action),
            value: Some(get_environment_value(action)),
            component: component_id.clone(),
        });

        // registry_items.push(Registry {
        //     registry: component_id.to_string(),
        //     name: "InstallDir".to_string(),
        //     key: "SOFTWARE\\Microsoft\\test".to_string(),
        //     value: action.value.to_string(),
        //     component: component_id.clone(),
        //     root: RegistryRoot::CurrentUser,
        // });

        feature_components.push(FeatureComponents {
            feature: "MainFeature".to_string(),
            component: component_id.clone(),
        });
    }

    Component::insert(package, &components)?;
    Environment::insert(package, &environments)?;
    // Registry::insert(package, &registry_items)?;
    FeatureComponents::insert(package, &feature_components)?;
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
