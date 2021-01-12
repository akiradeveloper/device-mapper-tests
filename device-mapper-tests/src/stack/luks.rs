use crate::{Stack, rand_name};
use tempfile::{NamedTempFile, TempPath};
use cmd_lib::run_cmd;
use std::io::Write;

pub struct Luks {
    name: String,
}
impl Luks {
    pub fn format(s: &impl Stack) {
        let path = s.path();
        let mut key_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(key_file, "aaaa").unwrap();
        let key_file_path = key_file.into_temp_path();
        let key_file_path = key_file_path.display();
        // -q: Don't ask for confirmation
        run_cmd!(cryptsetup luksFormat -q $path $key_file_path).unwrap();
    }
    pub fn new(s: &impl Stack) -> Self {
        let path = s.path();
        let name = rand_name();
        let mut key_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(key_file, "aaaa").unwrap();
        let key_file_path = key_file.into_temp_path();
        let key_file_path = key_file_path.display();
        run_cmd!(cryptsetup luksOpen $path $name --key-file=$key_file_path).unwrap();
        Self {
            name,
        }
    }
}
impl Stack for Luks {
    fn path(&self) -> String {
        format!("/dev/mapper/{}", self.name)
    }
}
impl Drop for Luks {
    fn drop(&mut self) {
        use std::time::Duration;
        
        let name = &self.name;
        // This looks like removing the device with --retry flag.
        run_cmd!(cryptsetup luksClose $name).unwrap();
    }
}