use device_mapper_tests::*;

use cmd_lib::{run_cmd, run_fun};

#[test]
fn test_wbcreate() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(wbcreate wbdev $p1 $p2 --reformat --read_cache_threshold=4 --writeback_threshold=70 --nr_max_batched_writeback=32 --update_sb_record_interval=3600 --sync_data_interval=3600 --write_around_mode).unwrap();
    run_cmd!(wbremove wbdev).unwrap();
}

#[test]
fn test_recreate() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(wbcreate wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(wbremove wbdev).unwrap();

    run_cmd!(wbcreate wbdev $p1 $p2).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();
    run_cmd!(wbremove wbdev).unwrap();
}

#[test]
fn test_wbremove() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(wbcreate wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(wbremove wbdev --noflush --nowriteback).unwrap();
}

#[test]
fn test_wbcheck() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(wbcreate wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(wbcheck $p2 1);

    run_cmd!(wbremove wbdev --nowriteback);
}

#[test]
fn test_wbdump() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(wbcreate wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(wbdump $p2 1).unwrap();

    run_cmd!(wbremove wbdev).unwrap();
}

#[test]
fn test_wbmeta() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(wbcreate wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(wbmeta $p2 0).unwrap();
    run_cmd!(wbmeta $p2 1).unwrap();

    run_cmd!(wbremove wbdev).unwrap();
}

#[test]
fn test_wbstatus() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(wbcreate wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(dmsetup status wbdev | wbstatus).unwrap();

    run_cmd!(wbremove wbdev --nowriteback).unwrap();
}
