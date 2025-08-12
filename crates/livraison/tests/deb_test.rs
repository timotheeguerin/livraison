use indoc::indoc;
use std::{fs, sync::LazyLock};
use test_macros::require_command;
mod test_utils;
use test_utils::{TestTempDir, exec};

use livraison::{
    common::FileRef,
    deb::{
        control::{Control, Priority, User},
        package::{DataFile, DebPackage},
    },
};

pub static TESTDIR: LazyLock<TestTempDir> = LazyLock::new(|| {
    let dir = TestTempDir::new("deb");
    dir.delete().expect("Worked");
    dir
});

fn ask_dpkg_deb_for_field(target: &str, field: &str) -> String {
    let output = exec("dpkg-deb", &["-f", target, field]);
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[require_command("dpkg-deb")]
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

    let dir = TESTDIR.mkdir("dpkg-deb").expect("Worked");
    let target_path_buf = dir.join("test.deb");

    let pkg = DebPackage {
        control: control.clone(),
        files: None,
        conf_files: None,
    };
    let file = fs::File::create(&target_path_buf).unwrap();
    pkg.write(file).unwrap();
    let target = target_path_buf.to_str().unwrap();

    let output = exec("dpkg-deb", &["-f", target]);
    println!("Created deb package at {target}");
    println!("{}", String::from_utf8(output.stdout).unwrap().trim());

    assert_eq!(ask_dpkg_deb_for_field(target, "Package"), control.package);
    assert_eq!(
        ask_dpkg_deb_for_field(target, "Version"),
        format!("{}-{}", control.version, control.revision.unwrap())
    );
    assert_eq!(
        ask_dpkg_deb_for_field(target, "Maintainer"),
        format!("{} <{}>", control.maintainer.name, control.maintainer.email)
    );
    assert_eq!(
        ask_dpkg_deb_for_field(target, "Architecture"),
        control.architecture
    );
    assert_eq!(
        ask_dpkg_deb_for_field(target, "Priority"),
        control.priority.unwrap().as_str(),
    );
    assert_eq!(
        ask_dpkg_deb_for_field(target, "Section"),
        control.section.unwrap(),
    );
}

#[require_command("lintian")]
#[test]
fn check_pass_lintian() {
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

    let dir = TESTDIR.mkdir("lintian").expect("Worked");
    let target_path_buf = dir.join("test.deb");

    let file = DataFile::new(
        "/etc/init.d/test",
        FileRef::from_text(indoc! {"
                #! /bin/sh
                do_start() {
                :
                }
                
                do_stop() {
                :
                }
                
                do_restart() {
                :
                }
                
                do_reload() {
                :
                }
                
                case $1 in
                start) do_start ;;
                stop) do_stop ;;
                force-reload) do_reload ;;
                esac    
                "})
        .with_mode(0o755),
    );
    let pkg = DebPackage {
        control: control.clone(),
        files: None,
        conf_files: Some(vec![file]),
    };
    let file = fs::File::create(&target_path_buf).unwrap();
    pkg.write(file).expect("Works");

    let target = target_path_buf.to_str().unwrap();

    let exclude = [
        "no-copyright-file",
        "no-changelog",
        "script-in-etc-init.d-not-registered-via-update-rc.d",
        "missing-systemd-service-for-init.d-script",
    ];
    let output = exec(
        "lintian",
        &["-i", target, "--suppress-tags", &exclude.join(",")],
    );
    if output.status.code().unwrap() != 0 {
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("{}", String::from_utf8(output.stderr).unwrap());
        panic!("Lintian failed");
    }
}
