use device_mapper_tests::*;

#[test]
fn test_compile_ruby_scenario() {
    use fs::xfs::*;
    use scenario::compile_ruby::*;

    let mut env = env();
    let dev = env.alloc_device(Sector::MB(1600));
    XFS::format(&dev);
    let fs = XFS::new(&dev);
    let cr = CompileRuby {
        root: &fs.mount_point,
    };
    cr.download();
    cr.unarchive();
    cr.compile();
    cr.check();
}
