#[non_exhaustive]
/// Cargo deb configuration read from the manifest and cargo metadata
pub struct Control {
    /// The package name
    pub package: String,
    /// The package version
    pub version: String,
    /// The package description
    pub description: String,
    /// The package maintainer
    pub maintainer: User,
    /// The package architecture
    pub architecture: String,
}

pub struct User {
    pub name: String,
    pub email: String,
}

/// Generates the control file that obtains all the important information about the package.
impl Control {
    pub fn write(&self) -> String {
        let out = vec![
            format!("Package: {}", self.package),
            format!("Version: {}", self.version),
            format!("Architecture: {}", self.architecture),
            self.write_description(),
            format!("Maintainer: {}", self.write_user(&self.maintainer)),
        ];

        return out.join("\n") + "\n";
    }

    fn write_description(&self) -> String {
        let mut out: String = "Description:".to_string();
        let mut first = true;
        for line in self.description.lines() {
            if !first {
                out.push('\n');
            }
            first = false;
            out.push_str(&format!(" {}", line));
        }
        out
    }

    fn write_user(&self, user: &User) -> String {
        return format!("{} <{}>", user.name, user.email);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn write_basic_control() {
        let control = Control {
            package: "test".to_string(),
            version: "1.0".to_string(),
            description: "A test package".to_string(),
            maintainer: User {
                name: "John Doe".to_string(),
                email: "john.doe@example.com".to_string(),
            },
            architecture: "all".to_string(),
        };
        let result = control.write();
        assert_eq!(
            result,
            indoc! { "
            Package: test
            Version: 1.0
            Architecture: all
            Description: A test package
            Maintainer: John Doe <john.doe@example.com>
            "}
            .to_string()
        );
    }
}
