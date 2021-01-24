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
    io::test_blk_rw(&dev, Sector(0), Sector(1));
    io::test_blk_rw(&dev, Sector(0), Sector::MB(50));
    io::test_blk_rw(&dev, Sector(0), Sector::MB(200));
}

#[test]
fn test_stat_purity() {
    let mut env = env();
    let slow = env.alloc_device(Sector::MB(500));
    let fast = env.alloc_device(Sector::MB(100));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default(),
    };

    // After create
    let dev = Writeboost::create(table);
    // We need to clear stat here because Device creation is followed
    // by some read I/O maybe to check the successful creation.
    dev.clear_stat();

    let stat = dev.status().stat;
    for i in 0..16 {
        let k = StatKey::from_bits(i);
        let v = *stat.get(&k).unwrap();
        assert_eq!(v, 0);
    }

    // After open:
    // Opening the device on the other hand doesn't do any side effects.
    let rw = open(&dev);
    let stat = dev.status().stat;
    for i in 0..16 {
        let k = StatKey::from_bits(i);
        let v = *stat.get(&k).unwrap();
        assert_eq!(v, 0);
    }

    let mut buf = io::buf(&vec![0; 4096]);
    rw.write(&buf, Sector(0), Sector::KB(4));
    let stat = dev.status().stat;
    let k1 = StatKey {
        write: true,
        hit: false,
        on_buffer: false,
        full_size: true,
    };
    for i in 0..16 {
        let k = StatKey::from_bits(i);
        let v = *stat.get(&k).unwrap();
        if k == k1 {
            assert_eq!(v, 1);
        } else {
            assert_eq!(v, 0);
        }
    }

    rw.read(&mut buf, Sector(0), Sector::KB(4));
    let stat = dev.status().stat;
    let k2 = StatKey {
        write: false,
        hit: true,
        on_buffer: true,
        full_size: true,
    };
    for i in 0..16 {
        let k = StatKey::from_bits(i);
        let v = *stat.get(&k).unwrap();
        if k == k1 || k == k2 {
            assert_eq!(v, 1);
        } else {
            assert_eq!(v, 0);
        }
    }
}