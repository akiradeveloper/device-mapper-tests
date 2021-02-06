use writeboost_tests::*;
use device_mapper_tests::*;

#[test]
fn test_rw() {
    let mut env = env();
    let slow = env.alloc_device(Sector::MB(500));
    let fast = env.alloc_device(Sector::MB(100));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default(),
    };
    let wb = Writeboost::create(table);
    io::test_blk_rw(&wb, Sector(0), Sector(1));
    io::test_blk_rw(&wb, Sector(0), Sector::MB(50));
    io::test_blk_rw(&wb, Sector(0), Sector::MB(200));
}

#[test]
fn test_full_options() {
    let mut env = env();
    let slow = env.alloc_device(Sector::MB(500));
    let fast = env.alloc_device(Sector::MB(100));
    sweep_caches(&fast);
    let options = Options::default()
        .writeback_threshold(100)
        .nr_max_batched_writeback(32)
        .update_sb_record_interval(3600)
        .sync_data_interval(3600)
        .read_cache_threshold(127)
        .write_around_mode()
        .nr_read_cache_cells(1);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options,
    };
    let _wb = Writeboost::create(table);
}