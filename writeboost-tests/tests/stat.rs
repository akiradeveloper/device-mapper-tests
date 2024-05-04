use device_mapper_tests::*;
use writeboost_tests::*;

// - drop_transient changes the stat as well as dm-create.
#[test]
fn test_stat_drop_transient() {
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
    wb.clear_stat();

    let st1 = wb.status().stat;
    wb.drop_transient();
    let st2 = wb.status().stat;
    assert_ne!(st1, st2);
}

// - drop_caches won't change the stat.
#[test]
fn test_stat_drop_caches() {
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
    wb.clear_stat();

    let st1 = wb.status().stat;
    wb.drop_caches();
    let st2 = wb.status().stat;
    assert_eq!(st1, st2);
}

// - dm-create changes the stat.
// - open doesn't.
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

    let dev = Writeboost::create(table);
    dev.clear_stat();

    let stat = dev.status().stat;
    for i in 0..16 {
        let k = StatKey::from_bits(i);
        let v = *stat.get(&k).unwrap();
        assert_eq!(v, 0);
    }

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
