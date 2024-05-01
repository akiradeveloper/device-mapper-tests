use crate::{rand_name, Stack};
use cmd_lib::run_cmd;

pub mod xfs;

fn mount(s: &impl Stack, options: &str) -> String {
    let blk_dev = s.path();
    let mp = format!("/tmp/{}", rand_name());
    run_cmd!(rm -rf $mp).unwrap();
    run_cmd!(mkdir -p $mp).unwrap();
    run_cmd!(mount $options $blk_dev $mp).unwrap();
    mp
}
fn umount(mount_point: &str) {
    run_cmd!(umount $mount_point).unwrap();
}
