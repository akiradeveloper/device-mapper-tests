use device_mapper_tests::*;
use writeboost_tests::*;

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
        let cr = CompileRuby {
            root: &fs.mount_point,
        };
        cr.download();
        cr.unarchive();

        kernel::drop_caches();
        wb.drop_transient();
    }

    {
        let table = Table {
            backing_dev: slow.path(),
            cache_dev: fast.path(),
            options: Options::default()
                .writeback_threshold(70)
                .read_cache_threshold(31),
        };
        let wb = Writeboost::create(table);
        let fs = XFS::new(&wb);
        let cr = CompileRuby {
            root: &fs.mount_point,
        };
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
        options: Options::default()
            .writeback_threshold(70)
            .read_cache_threshold(31),
    };
    let wb = Writeboost::create(table);
    XFS::format(&wb);
    let fs = XFS::new(&wb);
    let pwd = &fs.mount_point;
    let options = vec!["-t 60 1", "-S -t 60 4", "-s -t 60 4"];
    for option in options {
        let _pwd = tmp_env::set_current_dir(pwd).unwrap();
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
    let _pwd = tmp_env::set_current_dir(pwd).unwrap();
    run_cmd!(stress -v --timeout 60s --hdd 4 --hdd-bytes 1M).unwrap();
}
