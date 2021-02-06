use writeboost_tests::*;
use device_mapper_tests::*;

use cmd_lib::run_cmd;

#[test]
fn badblocks_default() {
    let mut env = env();
    let slow = env.alloc_device(Sector::MB(32));
    let fast = env.alloc_device(Sector::MB(8));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default(),
    };

    let wb = Writeboost::create(table);
    let p = wb.path();
    let r = run_cmd!(badblocks -vw $p);
    assert!(r.is_ok());
}

#[test]
fn badblocks_read_caching() {
    let mut env = env();
    let slow = env.alloc_device(Sector::MB(32));
    let fast = env.alloc_device(Sector::MB(8));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default().read_cache_threshold(1),
    };

    let wb = Writeboost::create(table);
    let p = wb.path();
    let r = run_cmd!(badblocks -vw $p);
    assert!(r.is_ok());
}