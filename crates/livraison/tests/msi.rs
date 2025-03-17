use std::{
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};
mod test_utils;
use test_utils::TestTempDir;

use livraison::msi::packer::{MsiInstallerOptions, pack};

pub static TESTDIR: LazyLock<TestTempDir> = LazyLock::new(|| {
    let dir = TestTempDir::new("msi");
    dir.delete().expect("Worked");
    dir
});

#[test]
fn check_dpkg_retrieve_information() {
    let options = MsiInstallerOptions {
        name: "test".to_string(),
        version: "1.0.0".to_string(),
        description: "Great test package\nWith nice description".to_string(),
        author: "John Smith".to_string(),
        icon: Some(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/computer.ico")
                .into_os_string()
                .into_string()
                .unwrap(),
        ),
        ..Default::default()
    };

    let dir = TESTDIR.mkdir("basic").expect("Worked");
    let msi_path = dir.join("basic.msi");

    pack(options.clone(), &msi_path).unwrap();
}
