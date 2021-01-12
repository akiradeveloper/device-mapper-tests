use cmd_lib::run_fun;
use crate::Sector;

pub fn getsize(path: &str) -> Sector {
    let output = run_fun!(blockdev --getsize $path).unwrap();
    let n = output.parse().unwrap();
    Sector(n)
}