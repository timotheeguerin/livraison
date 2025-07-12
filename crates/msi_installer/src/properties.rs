// Manage the property table

use msi::{Language, Package};
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use strum_macros::EnumString;
use uuid::Uuid;

use crate::tables::{Entity, Property};

#[derive(Debug)]
pub struct RequiredProperties {
    pub product_code: Uuid,
    pub product_language: Language,
    pub manufacturer: String,
    pub product_name: String,
    pub product_version: String,
}
#[derive(Debug)]
pub struct PropertiesBuilder {
    properties: HashMap<PropertyType, String>,
}

fn guid_string(value: &Uuid) -> String {
    format!("{{{value}}}")
}

impl PropertiesBuilder {
    pub fn new(props: RequiredProperties) -> Self {
        let mut properties = HashMap::new();
        properties.insert(PropertyType::ProductCode, guid_string(&props.product_code));
        properties.insert(
            PropertyType::ProductLanguage,
            props.product_language.code().to_string(),
        );
        properties.insert(PropertyType::Manufacturer, props.manufacturer);
        properties.insert(PropertyType::ProductName, props.product_name);
        properties.insert(PropertyType::ProductVersion, props.product_version);
        Self { properties }
    }

    pub fn upgrade_code(&mut self, value: &Uuid) -> &mut Self {
        self.properties
            .insert(PropertyType::UpgradeCode, guid_string(value));
        self
    }

    /// Install per user $LOCALAPPDATA/Programs
    pub fn install_per_user(&mut self) -> &mut Self {
        self.properties
            .insert(PropertyType::AllUsers, "2".to_string());
        self.properties
            .insert(PropertyType::MsiInstallPerUser, "1".to_string());
        self
    }

    /// Install globally $PROGRAMFILES/Programs
    pub fn install_global(&mut self) -> &mut Self {
        self.properties.remove(&PropertyType::AllUsers);
        self.properties.remove(&PropertyType::MsiInstallPerUser);
        self
    }

    pub fn default_ui_font(&mut self, value: &str) -> &mut Self {
        self.properties
            .insert(PropertyType::DefaultUIFont, value.to_string());
        self
    }

    /// Setting property disables Add or Remove Programs functionality in Control Panel that modifies the product.
    /// https://learn.microsoft.com/en-us/windows/win32/msi/arpnomodify
    pub fn arp_no_modify(&mut self, value: bool) -> &mut Self {
        if value {
            self.properties
                .insert(PropertyType::ArpNoModify, value.to_string());
        } else {
            self.properties.remove(&PropertyType::ArpNoModify);
        }
        self
    }

    pub fn insert(&mut self, property: &str, value: &str) -> &mut Self {
        self.properties.insert(
            PropertyType::Custom(property.to_string()),
            value.to_string(),
        );
        self
    }

    pub fn build(&self) -> Vec<Property> {
        self.properties
            .iter()
            .map(|(property, value)| Property {
                property: property.to_string(),
                value: value.to_string(),
            })
            .collect()
    }

    pub fn create_table<F: Read + Seek + Write>(
        &self,
        package: &mut Package<F>,
    ) -> Result<(), std::io::Error> {
        Property::create_table(package)?;
        let rows = self.build();
        Property::insert(package, &rows)?;
        Ok(())
    }
}

// cspell:ignore MSIINSTALLPERUSER ARPNOMODIFY
#[derive(Debug, PartialEq, Eq, Hash, EnumString, strum_macros::Display)]
// #[allow(dead_code)]
enum PropertyType {
    // Required properties
    ProductCode,
    ProductLanguage,
    Manufacturer,
    ProductName,
    ProductVersion,

    // Other
    UpgradeCode,
    DefaultUIFont,
    #[strum(serialize = "ARPNOMODIFY")]
    ArpNoModify,
    #[strum(serialize = "ALLUSERS")]
    AllUsers,
    #[strum(serialize = "MSIINSTALLPERUSER")]
    MsiInstallPerUser,
    #[strum(to_string = "{0}")]
    Custom(String),
}
