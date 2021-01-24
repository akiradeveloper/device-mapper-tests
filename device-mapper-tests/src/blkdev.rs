use cmd_lib::run_fun;
use crate::Sector;

pub fn get_size(path: &str) -> Sector {
    let output = run_fun!(blockdev --getsize $path).unwrap();
    let n = output.parse().unwrap();
    Sector(n)
}

pub fn get_blocksize(path: &str) -> u64 {
    let output = run_fun!(blockdev --getss $path).unwrap();
    let n: u64 = output.parse().unwrap();
    n
}