use writeboost_tests::*;
use device_mapper_tests::*;

use cmd_lib::*;

#[test]
fn badblocks() {
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
fn comipile_ruby() {
    use fs::xfs::*;
    use scenario::compile_ruby::*;

    let mut env = env();
    let slow = env.alloc_device(Sector::MB(1600));
    let fast = env.alloc_device(Sector::MB(300));
    sweep_caches(&fast);

    {
        let table = Table {
            backing_dev: slow.path(),
            cache_dev: fast.path(),
            options: Options::default(),
        };
        let wb = Writeboost::create(table);
        XFS::format(&wb);
        let fs = XFS::new(&wb);
        let cr = CompileRuby { root: &fs.mount_point };
        cr.download();
        cr.unarchive();

        kernel::drop_caches();
        wb.drop_transient();
    }

    {
        let table = Table {
            backing_dev: slow.path(),
            cache_dev: fast.path(),
            options: Options::default().writeback_threshold(70).read_cache_threshold(31),
        };
        let wb = Writeboost::create(table);
        let fs = XFS::new(&wb);
        let cr = CompileRuby { root: &fs.mount_point };
        cr.compile();
        cr.check();
    }
}

#[test]
fn dbench() {
    use fs::xfs::*;

    let mut env = env();
    let slow = env.alloc_device(Sector::MB(1600));
    let fast = env.alloc_device(Sector::MB(300));
    sweep_caches(&fast);

    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default().writeback_threshold(70).read_cache_threshold(31),
    };
    let wb = Writeboost::create(table);
    XFS::format(&wb);
    let fs = XFS::new(&wb);
    let pwd = &fs.mount_point;
    let option = "-t 60 1";
    proc_env_set!(PWD = pwd);
    {
        // We use run_cmd function instead of macro here
        // https://github.com/rust-shell-script/rust_cmd_lib/issues/17
        run_cmd(format!("dbench {}", option)).unwrap();
    }
}

#[test]
fn stress() {
    use fs::xfs::*;

    let mut env = env();
    let slow = env.alloc_device(Sector::MB(1600));
    let fast = env.alloc_device(Sector::MB(300));
    sweep_caches(&fast);

    let table = Table {
        backing_dev: slow.path(),
        cache_dev: fast.path(),
        options: Options::default(),
    };
    let wb = Writeboost::create(table);
    XFS::format(&wb);
    let fs = XFS::new(&wb);
    let pwd = &fs.mount_point; 
    // 4 workers
    // each worker writes 256MB
    proc_env_set!(PWD = pwd);
    {
        run_cmd!(stress -v --timeout 60 --hdd 4 --hdd-bytes 256M).unwrap();
    }
} 