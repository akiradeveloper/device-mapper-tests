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
    let dev = Writeboost::create(table);
    io::test_blk_rw(&dev, Sector(0), Sector::KB(1));
    io::test_blk_rw(&dev, Sector(0), Sector::MB(50));
    io::test_blk_rw(&dev, Sector(0), Sector::MB(200));
}