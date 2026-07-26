use std::{fs, sync::LazyLock};
use test_macros::require_command;
mod test_utils;
use test_utils::{TestTempDir, exec};

use livraison::{
    common::FileRef,
    pkg::{
        metadata::{PkgMetadata, User},
        package::{DataFile, PkgPackage},
    },
};

pub static TESTDIR: LazyLock<TestTempDir> = LazyLock::new(|| {
    let dir = TestTempDir::new("pkg");
    dir.delete().expect("Worked");
    dir
});

fn mk_metadata() -> PkgMetadata {
    PkgMetadata {
        identifier: "com.livraison.test".to_string(),
        name: "test".to_string(),
        version: "1.2.3".to_string(),
        install_location: "/".to_string(),
        author: User {
            name: "John Smith".to_string(),
            email: "john.smith@example.com".to_string(),
        },
    }
}

fn write_package(dir_name: &str, pkg: &PkgPackage) -> std::path::PathBuf {
    let dir = TESTDIR.mkdir(dir_name).expect("Worked");
    let target_path_buf = dir.join("test.pkg");
    let file = fs::File::create(&target_path_buf).unwrap();
    pkg.write(file).unwrap();
    target_path_buf
}

fn simple_pkg() -> PkgPackage {
    PkgPackage {
        metadata: mk_metadata(),
        files: vec![DataFile::new(
            "/usr/local/bin/test",
            FileRef::from_text("#!/bin/sh\necho hello\n").with_mode(0o100755),
        )],
    }
}

#[require_command("xar")]
#[test]
fn check_pkg_xar_contains_expected_members() {
    let target_path_buf = write_package("xar", &simple_pkg());
    let target = target_path_buf.to_str().unwrap();

    let output = exec("xar", &["-tf", target]);
    let listing = String::from_utf8(output.stdout).unwrap();
    for member in ["Bom", "PackageInfo", "Payload"] {
        assert!(listing.contains(member), "xar listing was: {listing}");
    }
}

#[require_command("xar")]
#[require_command("lsbom")]
#[test]
fn check_pkg_bom_lists_files_with_cksum() {
    let dir = TESTDIR.mkdir("bom").expect("Worked");
    let target_path_buf = dir.join("test.pkg");
    let file = fs::File::create(&target_path_buf).unwrap();
    simple_pkg().write(file).unwrap();

    // Extract the Bom out of the xar archive.
    exec(
        "xar",
        &[
            "-xf",
            target_path_buf.to_str().unwrap(),
            "-C",
            dir.to_str().unwrap(),
        ],
    );
    let bom = dir.join("Bom");
    let output = exec("lsbom", &[bom.to_str().unwrap()]);
    let listing = String::from_utf8(output.stdout).unwrap();

    assert!(
        listing.contains("./usr/local/bin/test"),
        "lsbom output was: {listing}"
    );
    // The BOM records the POSIX `cksum` checksum of the payload file.
    assert!(
        listing.contains("1294090613"),
        "expected cksum checksum in lsbom output: {listing}"
    );
}

#[require_command("xar")]
#[test]
fn check_pkg_payload_is_gzip_cpio() {
    let dir = TESTDIR.mkdir("payload").expect("Worked");
    let target_path_buf = dir.join("test.pkg");
    let file = fs::File::create(&target_path_buf).unwrap();
    simple_pkg().write(file).unwrap();

    exec(
        "xar",
        &[
            "-xf",
            target_path_buf.to_str().unwrap(),
            "-C",
            dir.to_str().unwrap(),
        ],
    );
    let payload = dir.join("Payload");
    let bytes = fs::read(&payload).unwrap();
    // gzip magic.
    assert_eq!(&bytes[0..2], &[0x1f, 0x8b], "Payload should be gzip");
}

#[require_command("pkgutil")]
#[test]
fn check_pkgutil_lists_payload_files() {
    let target_path_buf = write_package("pkgutil", &simple_pkg());
    let target = target_path_buf.to_str().unwrap();

    let output = exec("pkgutil", &["--payload-files", target]);
    let listing = String::from_utf8(output.stdout).unwrap();
    assert!(
        listing.contains("usr/local/bin/test"),
        "pkgutil payload listing was: {listing}"
    );
}
