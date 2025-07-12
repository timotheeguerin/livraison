use std::{fs, path::PathBuf, process::Command};

pub struct TestTempDir {
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

    pub fn mkdir(&self, name: &str) -> std::io::Result<PathBuf> {
        let dir = self.base_dir.join(name);
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}

#[allow(dead_code)]
pub fn exec(command: &str, args: &[&str]) -> std::process::Output {
    Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command")
}
