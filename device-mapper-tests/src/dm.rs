use std::process::Command;
use std::io::Write;
use std::str::FromStr;
use cmd_lib::{run_cmd, run_fun};
use crate::Sector;

pub struct Table {
    pub start: Sector,
    pub len: Sector,
    pub target: String,
    pub args: Vec<String>,
}
impl FromStr for Table {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let xs: Vec<&str> = s.split(" ").collect();
        let start = Sector(xs[0].parse().unwrap());
        let len = Sector(xs[1].parse().unwrap());
        let target = xs[2].to_owned();
        let mut args = vec![];
        for &x in &xs[3..] {
            args.push(x.to_owned())
        }
        Ok(Table { start, len, target, args, })
    }
}
pub struct Status {
    pub start: Sector,
    pub len: Sector,
    pub target: String,
    pub args: Vec<String>,
}
impl FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let xs: Vec<&str> = s.split(" ").collect();
        let start = Sector(xs[0].parse().unwrap());
        let len = Sector(xs[1].parse().unwrap());
        let target = xs[2].to_owned();
        let mut args = vec![];
        for &x in &xs[3..] {
            args.push(x.to_owned())
        }
        Ok(Status { start, len, target, args, })
    }
}
pub struct State {
    name: String,
}
impl State {
    pub fn new(name: String) -> Self {
        State {
            name,
        }
    }
    pub fn path(&self) -> String {
        format!("/dev/mapper/{}", self.name)
    }
    pub fn create(&self) { 
        let name = &self.name;
        run_cmd!(dmsetup create $name --notable).unwrap();
    }
    pub fn remove(&self) {
        use std::time::Duration;

        let name = &self.name;
        // Removing device often fails due to resource busy.
        // The reason is not sure but it may be because the close is a bit lazy.
        run_cmd!(dmsetup remove --retry $name).unwrap();
    }
    pub fn reload(&self, table_line: &str) {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "{}", table_line).unwrap();
        let path = f.into_temp_path();
        let path = path.display();
        let name = &self.name;
        run_cmd!(dmsetup reload $name $path).unwrap();
    }
    pub fn table(&self) -> Table {
        let name = &self.name;
        let output = run_fun!(dmsetup table $name).unwrap();
        output.parse().unwrap()
    }
    pub fn status(&self) -> Status {
        let name = &self.name;
        let output = run_fun!(dmsetup status $name).unwrap();
        output.parse().unwrap()
    }
    pub fn message(&self, msg: &str) {
        let name = &self.name;
        run_cmd!(dmsetup message $name 0 $msg).unwrap();
    }
    pub fn suspend(&self) {
        let name = &self.name;
        run_cmd!(dmsetup suspend $name).unwrap();
    }
    pub fn resume(&self) {
        let name = &self.name;
        run_cmd!(dmsetup resume $name).unwrap();
    }
}