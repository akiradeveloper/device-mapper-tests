use cmd_lib::run_cmd;

pub fn drop_caches() {
    run_cmd!(sysctl -w vm.drop_caches=3).unwrap()
}