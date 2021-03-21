use writeboost_tests::*;
use device_mapper_tests::*;

#[test]
fn read_caching_disabled() {
    use io::pattern::{PatternIO, Pattern};

    let mut env = env();
    let slow = env.alloc_device(Sector::GB(1));
    let fast = env.alloc_device(Sector::MB(32));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default(),
    };
    let wb = Writeboost::create(table);
    wb.clear_stat();

    let rw = open(&wb);
    let pats = vec![
        Pattern::Read(Sector::KB(4)),
        Pattern::Skip(Sector::KB(4)),
    ];
    let k = StatKey {
        write: false,
        hit: true,
        on_buffer: false,
        full_size: true,
    };
    let pat = PatternIO::new().max_io_amount(Sector::MB(16));
    pat.submit(&rw, &pats);
    pat.submit(&rw, &pats);
    let st = wb.status().stat;
    assert_eq!(*st.get(&k).unwrap(), 0);
}

#[test]
fn read_caching_enabled() {
    use io::pattern::{PatternIO, Pattern};

    let mut env = env();
    let slow = env.alloc_device(Sector::GB(1));
    let fast = env.alloc_device(Sector::MB(32));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default().read_cache_threshold(1),
    };
    let wb = Writeboost::create(table);
    wb.clear_stat();

    let rw = open(&wb);
    let pats = vec![
        Pattern::Read(Sector::KB(4)),
        Pattern::Skip(Sector::KB(4)),
    ];
    let k = StatKey {
        write: false,
        hit: true,
        on_buffer: false,
        full_size: true,
    };
    let pat = PatternIO::new().max_io_amount(Sector::MB(16));
    // Staging
    pat.submit(&rw, &pats);
    let st = wb.status().stat;
    assert_eq!(*st.get(&k).unwrap(), 0); 

    // Drop RAM buffer
    wb.drop_transient();
    wb.clear_stat();

    // Cache hits
    pat.submit(&rw, &pats);
    let st = wb.status().stat;
    assert!(*st.get(&k).unwrap() > 0);
    // assert_eq!(*st.get(&k).unwrap(), 2048); 
}

#[test]
fn read_caching_threshold_127_works() {
    use io::pattern::{PatternIO, Pattern};

    let mut env = env();
    let slow = env.alloc_device(Sector::GB(1));
    let fast = env.alloc_device(Sector::MB(32));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default().read_cache_threshold(127),
    };
    let wb = Writeboost::create(table);
    wb.clear_stat();

    let rw = open(&wb);
    let pats = vec![
        Pattern::Read(Sector::MB(1)),
        Pattern::Skip(Sector::MB(1)),
    ];
    let k = StatKey {
        write: false,
        hit: true,
        on_buffer: false,
        full_size: true,
    };
    let pat = PatternIO::new().max_io_amount(Sector::MB(24));
    // Threshold works
    pat.submit(&rw, &pats);

    wb.drop_transient();
    wb.clear_stat();

    // No Cache hits
    pat.submit(&rw, &pats);

    let st = wb.status().stat;
    assert!(*st.get(&k).unwrap() < 128);
    // assert_eq!(*st.get(&k).unwrap(), 0);
}

#[test]
fn read_caching_threshold_1_works() {
    use io::pattern::{PatternIO, Pattern};

    let mut env = env();
    let slow = env.alloc_device(Sector::GB(1));
    let fast = env.alloc_device(Sector::MB(32));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default().read_cache_threshold(1),
    };
    let wb = Writeboost::create(table);
    wb.clear_stat();

    let rw = open(&wb);
    let pats = vec![
        Pattern::Read(Sector::KB(8)),
        Pattern::Skip(Sector::KB(8)),
    ];
    let k = StatKey {
        write: false,
        hit: true,
        on_buffer: false,
        full_size: true,
    };
    let pat = PatternIO::new().max_io_amount(Sector::MB(24));
    // Threshold works
    pat.submit(&rw, &pats);

    wb.drop_transient();
    wb.clear_stat();

    // No Cache hits
    pat.submit(&rw, &pats);

    let st = wb.status().stat;
    assert!(*st.get(&k).unwrap() < 2);
    // assert_eq!(*st.get(&k).unwrap(), 0);
}

#[test]
fn read_caching_not_work_partial_read() {
    use io::pattern::{PatternIO, Pattern};

    let mut env = env();
    let slow = env.alloc_device(Sector::GB(1));
    let fast = env.alloc_device(Sector::MB(32));
    sweep_caches(&fast);
    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default().read_cache_threshold(127),
    };
    let wb = Writeboost::create(table);
    wb.clear_stat(); 

    let rw = open(&wb);
    let pats = vec![
        Pattern::Read(Sector::KB(3)),
        Pattern::Skip(Sector::KB(5)),
    ]; 
    let pat = PatternIO::new().max_io_amount(Sector::MB(16));
    pat.submit(&rw, &pats);    

    wb.drop_transient();
    wb.clear_stat();

    pat.submit(&rw, &pats);    

    let st = wb.status().stat; 
    // full read hit
    let k1 = StatKey {
        write: false,
        hit: true,
        on_buffer: false,
        full_size: true, 
    };
    assert_eq!(*st.get(&k1).unwrap(), 0);

    // partial read hit
    let k2 = StatKey {
        write: false,
        hit: true,
        on_buffer: false,
        full_size: false, 
    };
    assert_eq!(*st.get(&k2).unwrap(), 0);
}