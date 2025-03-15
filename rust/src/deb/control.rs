#[non_exhaustive]
/// Cargo deb configuration read from the manifest and cargo metadata
#[derive(Default, Debug)]
pub struct Control {
    /// The package name
    pub package: String,
    /// Version number of the package, used in the Version field of the control specification.
    /// Only the upstream version part of the whole debian package version.
    /// https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-version
    pub version: String,
    /// This is a single (generally small) unsigned integer. It may be omitted, in which case zero is assumed.
    pub epoch: Option<u32>,
    /// This part of the version number specifies the version of the Debian package based on the upstream version.
    pub revision: Option<String>,

    /// The package description
    pub description: String,
    /// The package maintainer
    pub maintainer: User,
    /// The package architecture
    pub architecture: String,

    /// Application area into which the package has been classified.
    pub section: Option<String>,
    /// Application area into which the package has been classified.
    pub priority: Option<Priority>,

    /// Relationships to other packages, used in the Depends field of the control specification.
    pub depends: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum Priority {
    Required,
    Important,
    Standard,
    Optional,
}

impl Priority {
    fn as_str(&self) -> &'static str {
        match self {
            Priority::Required => "required",
            Priority::Important => "important",
            Priority::Standard => "standard",
            Priority::Optional => "optional",
        }
    }
}

#[derive(Default, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
}

/// Generates the control file that obtains all the important information about the package.
impl Control {
    pub fn write(&self) -> String {
        let mut out = vec![
            format!("Package: {}", self.package),
            self.write_version(),
            format!("Architecture: {}", self.architecture),
            self.write_description(),
            format!("Maintainer: {}", self.format_user(&self.maintainer)),
        ];

        if let Some(priority) = &self.priority {
            out.push(format!("Priority: {}", priority.as_str()));
        }

        if let Some(section) = &self.section {
            out.push(format!("Section: {}", section));
        }
        if let Some(depends) = &self.depends {
            out.push(format!("Depends: {}", self.format_depends(depends)));
        }

        return out.join("\n") + "\n";
    }

    fn write_version(&self) -> String {
        let mut out: String = "Version: ".to_string();
        if let Some(epoch) = &self.epoch {
            out.push_str(&format!("{}:", epoch));
        }
        out.push_str(&self.version);
        if let Some(revision) = &self.revision {
            out.push('-');
            out.push_str(revision);
        }
        out
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

    fn format_user(&self, user: &User) -> String {
        return format!("{} <{}>", user.name, user.email);
    }

    fn format_depends(&self, items: &Vec<String>) -> String {
        return items.join(", ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::{self, assert_contains};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn mk_control() -> Control {
        return Control {
            package: "test".to_string(),
            version: "1.0".to_string(),
            description: "A test package".to_string(),
            maintainer: User {
                name: "John Doe".to_string(),
                email: "john.doe@example.com".to_string(),
            },
            architecture: "all".to_string(),
            ..Default::default()
        };
    }
    #[test]
    fn write_basic_control() {
        let control = mk_control();
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

    #[test]
    fn render_version_with_epoch_and_revision() {
        let control = Control {
            epoch: Some(1),
            revision: Some("23".to_string()),
            ..mk_control()
        };
        let result = control.write();
        assert_contains!(result, "Version: 1:1.0-23\n");
    }
    #[test]
    fn render_section() {
        let control = Control {
            section: Some("misc".to_string()),
            ..mk_control()
        };
        let result = control.write();
        assert_contains!(result, "Section: misc\n");
    }
    #[test]
    fn render_priority() {
        let control = Control {
            priority: Some(Priority::Important),
            ..mk_control()
        };

        let result = control.write();
        assert_contains!(result, "Priority: important\n");
    }

    #[test]
    fn render_depends() {
        let control = Control {
            depends: Some(vec!["libfoo > 1".to_string(), "libbar".to_string()]),
            ..mk_control()
        };

        let result = control.write();
        assert_contains!(result, "Depends: libfoo > 1,libbar\n");
    }
}
