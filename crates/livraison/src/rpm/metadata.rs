//! Package metadata for an RPM, analogous to the deb `Control`.

/// A package author / packager.
#[derive(Default, Debug, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl User {
    /// Format as `Name <email>`, or just the name when no email is set.
    pub fn format(&self) -> String {
        if self.email.is_empty() {
            self.name.clone()
        } else {
            format!("{} <{}>", self.name, self.email)
        }
    }
}

/// Metadata describing an RPM package. Feeds the main header tags.
#[derive(Debug, Clone)]
pub struct RpmMetadata {
    /// Package name.
    pub name: String,
    /// Upstream version (no dashes allowed by RPM).
    pub version: String,
    /// Package release / revision.
    pub release: String,
    /// One-line summary.
    pub summary: String,
    /// Long description.
    pub description: String,
    /// License identifier.
    pub license: String,
    /// Target architecture (e.g. `noarch`, `x86_64`).
    pub arch: String,
    /// Packager.
    pub packager: User,
}

impl Default for RpmMetadata {
    fn default() -> Self {
        RpmMetadata {
            name: String::new(),
            version: "1.0.0".to_string(),
            release: "1".to_string(),
            summary: String::new(),
            description: String::new(),
            license: "Unknown".to_string(),
            arch: "noarch".to_string(),
            packager: User::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_format_with_email() {
        let user = User {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        assert_eq!(user.format(), "John Doe <john@example.com>");
    }

    #[test]
    fn user_format_without_email() {
        let user = User {
            name: "John Doe".to_string(),
            email: String::new(),
        };
        assert_eq!(user.format(), "John Doe");
    }
}
