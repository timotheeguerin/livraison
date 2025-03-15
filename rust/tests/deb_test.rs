use std::{fs, path::PathBuf, process::Command, sync::LazyLock};

use livraison::deb::{
    control::{Control, Priority, User},
    deb_package::DebPackage,
};

struct TestTempDir {
    pub base_dir: PathBuf,
}

impl TestTempDir {
    pub fn new(name: &str) -> Self {
        let temp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp/test");
        let base_dir = temp_dir.join(name);
        TestTempDir { base_dir }
    }

    pub fn delete(&self) -> std::io::Result<()> {
        if self.base_dir.exists() {
            fs::remove_dir_all(&self.base_dir)?;
        }
        Ok(())
    }

    pub fn mkdir(&self) -> std::io::Result<PathBuf> {
        let r: String = (0..7)
            .map(|_| (0x61u8 + (rand::random::<u8>() % 26)) as char)
            .collect();
        let dir = self.base_dir.join(r);
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}

static TESTDIR: LazyLock<TestTempDir> = LazyLock::new(|| {
    let dir = TestTempDir::new("deb");
    dir.delete().expect("Worked");
    dir
});

fn exec(command: &str, args: &[&str]) -> std::process::Output {
    Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command")
}

fn ask_dpkg_deb_for_field(target: &str, field: &str) -> String {
    let output = exec("dpkg-deb", &["-f", target, field]);
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn check_dpkg_retrieve_information() {
    let control = Control {
        package: "test".to_string(),
        version: "1.0.0".to_string(),
        revision: Some("12".to_string()),
        description: "Great test package\nWith nice description".to_string(),
        architecture: "all".to_string(),
        priority: Some(Priority::Optional),
        section: Some("misc".to_string()),
        maintainer: User {
            name: "John Smith".to_string(),
            email: "john.smith@example.com".to_string(),
        },
        depends: Some(vec!["libc6".to_string(), "libstdc++6".to_string()]),
        ..Default::default()
    };

    let dir = TESTDIR.mkdir().expect("Worked");
    let target_path_buf = dir.join("test.deb");

    let pkg = DebPackage {
        control: control.clone(),
    };
    let file = fs::File::create(&target_path_buf).unwrap();
    pkg.write(file).unwrap();
    let target = target_path_buf.to_str().unwrap();

    let output = exec("dpkg-deb", &["-f", target]);
    println!("Created deb package at {}", target);
    println!("{}", String::from_utf8(output.stdout).unwrap().trim());

    assert_eq!(ask_dpkg_deb_for_field(&target, "Package"), control.package);
    assert_eq!(
        ask_dpkg_deb_for_field(&target, "Version"),
        format!("{}-{}", control.version, control.revision.unwrap())
    );
    assert_eq!(
        ask_dpkg_deb_for_field(&target, "Maintainer"),
        format!("{} <{}>", control.maintainer.name, control.maintainer.email)
    );
    assert_eq!(
        ask_dpkg_deb_for_field(&target, "Architecture"),
        control.architecture
    );
    assert_eq!(
        ask_dpkg_deb_for_field(&target, "Priority"),
        control.priority.unwrap().as_str(),
    );
    assert_eq!(
        ask_dpkg_deb_for_field(&target, "Section"),
        control.section.unwrap(),
    );
}
