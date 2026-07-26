//! Metadata for a macOS flat *component* installer package (`.pkg`).

/// Product author / packager.
#[derive(Debug, Default, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

/// Metadata describing a single-component macOS installer package.
#[derive(Debug, Clone)]
pub struct PkgMetadata {
    /// Reverse-DNS package identifier, e.g. `com.livraison.mytool`.
    pub identifier: String,

    /// Human readable product name.
    pub name: String,

    /// Product version, e.g. `1.0.0`.
    pub version: String,

    /// Absolute prefix the payload is relative to. Payload paths are stored
    /// relative to this (typically `/`).
    pub install_location: String,

    /// Product author / packager.
    pub author: User,
}

impl Default for PkgMetadata {
    fn default() -> Self {
        PkgMetadata {
            identifier: "com.livraison.package".to_string(),
            name: "package".to_string(),
            version: "0.0.0".to_string(),
            install_location: "/".to_string(),
            author: User::default(),
        }
    }
}
