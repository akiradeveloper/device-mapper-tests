use writeboost_tests::*;
use device_mapper_tests::*;

use cmd_lib::run_cmd;

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
    }

    {
        let table = Table {
            backing_dev: slow.path(),
            cache_dev: fast.path(),
            options: Options::default().writeback_threshold(70).read_cache_threshold(31),
        };
        let wb = Writeboost::create(table);
        XFS::format(&wb);
        let fs = XFS::new(&wb);
        let cr = CompileRuby { root: &fs.mount_point };
        cr.compile();
        cr.check();
    }
}