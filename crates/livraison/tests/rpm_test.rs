use std::{fs, sync::LazyLock};
use test_macros::require_command;
mod test_utils;
use test_utils::{TestTempDir, exec};

use livraison::{
    common::FileRef,
    rpm::{
        metadata::{RpmMetadata, User},
        package::{DataFile, RpmPackage},
    },
};

pub static TESTDIR: LazyLock<TestTempDir> = LazyLock::new(|| {
    let dir = TestTempDir::new("rpm");
    dir.delete().expect("Worked");
    dir
});

fn ask_rpm_for_field(target: &str, format: &str) -> String {
    let output = exec("rpm", &["-qp", "--queryformat", format, target]);
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn mk_metadata() -> RpmMetadata {
    RpmMetadata {
        name: "test".to_string(),
        version: "1.0.0".to_string(),
        release: "1".to_string(),
        summary: "Great test package".to_string(),
        description: "Great test package\nWith nice description".to_string(),
        license: "MIT".to_string(),
        arch: "noarch".to_string(),
        packager: User {
            name: "John Smith".to_string(),
            email: "john.smith@example.com".to_string(),
        },
    }
}

fn write_package(dir_name: &str, pkg: &RpmPackage) -> std::path::PathBuf {
    let dir = TESTDIR.mkdir(dir_name).expect("Worked");
    let target_path_buf = dir.join("test.rpm");
    let file = fs::File::create(&target_path_buf).unwrap();
    pkg.write(file).unwrap();
    target_path_buf
}

#[require_command("rpm")]
#[test]
fn check_rpm_retrieve_information() {
    let metadata = mk_metadata();
    let pkg = RpmPackage {
        metadata: metadata.clone(),
        files: vec![DataFile::new(
            "/usr/local/bin/test",
            FileRef::from_text("#!/bin/sh\necho hello\n").with_mode(0o100755),
        )],
    };
    let target_path_buf = write_package("query", &pkg);
    let target = target_path_buf.to_str().unwrap();

    assert_eq!(ask_rpm_for_field(target, "%{NAME}"), metadata.name);
    assert_eq!(ask_rpm_for_field(target, "%{VERSION}"), metadata.version);
    assert_eq!(ask_rpm_for_field(target, "%{RELEASE}"), metadata.release);
    assert_eq!(ask_rpm_for_field(target, "%{ARCH}"), metadata.arch);
    assert_eq!(ask_rpm_for_field(target, "%{LICENSE}"), metadata.license);
    assert_eq!(ask_rpm_for_field(target, "%{SUMMARY}"), metadata.summary);
    assert_eq!(
        ask_rpm_for_field(target, "%{DESCRIPTION}"),
        metadata.description
    );
    assert_eq!(
        ask_rpm_for_field(target, "%{PACKAGER}"),
        format!("{} <{}>", metadata.packager.name, metadata.packager.email)
    );
}

#[require_command("rpm")]
#[test]
fn check_rpm_lists_payload_files() {
    let pkg = RpmPackage {
        metadata: mk_metadata(),
        files: vec![
            DataFile::new(
                "/usr/local/bin/test",
                FileRef::from_text("#!/bin/sh\necho hello\n").with_mode(0o100755),
            ),
            DataFile::new(
                "/usr/local/bin/other",
                FileRef::from_text("#!/bin/sh\necho other\n").with_mode(0o100755),
            ),
        ],
    };
    let target_path_buf = write_package("files", &pkg);
    let target = target_path_buf.to_str().unwrap();

    let output = exec("rpm", &["-qlp", target]);
    let listing = String::from_utf8(output.stdout).unwrap();
    assert!(
        listing.contains("/usr/local/bin/test"),
        "listing was: {listing}"
    );
    assert!(
        listing.contains("/usr/local/bin/other"),
        "listing was: {listing}"
    );
}

#[require_command("rpm")]
#[test]
fn check_rpm_header_digest_is_valid() {
    let pkg = RpmPackage {
        metadata: mk_metadata(),
        files: vec![DataFile::new(
            "/usr/local/bin/test",
            FileRef::from_text("#!/bin/sh\necho hello\n").with_mode(0o100755),
        )],
    };
    let target_path_buf = write_package("digest", &pkg);
    let target = target_path_buf.to_str().unwrap();

    // `rpm -K` verifies the header and payload SHA256 digests. Without a GPG
    // key it reports missing signatures but must not report a digest failure.
    let output = exec("rpm", &["-Kv", target]);
    let report = String::from_utf8(output.stdout).unwrap();
    assert!(
        report.contains("Header SHA256 digest: OK"),
        "checksig report was: {report}"
    );
    assert!(
        report.contains("Payload SHA256 digest: OK"),
        "checksig report was: {report}"
    );
}
