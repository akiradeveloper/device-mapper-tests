use crate::{Stack, Sector};
use cmd_lib::{run_fun, run_cmd};

pub struct MemBlk {
    path: String,
    loop_device: String,
}
impl MemBlk {
    pub fn new(env: &mut MemBlkAllocator, mb: u64) -> Self {
        let path = env.alloc();
        run_cmd!(dd status=none if=/dev/zero of=$path bs=1M count=$mb).unwrap();
        let loop_device = run_fun!(losetup -f).unwrap();
        run_cmd!(losetup --sector-size=512 $loop_device $path).unwrap();
        dbg!(run_fun!(blockdev --getss $loop_device));
        Self {
            path,
            loop_device,
        }
    }
}
impl Stack for MemBlk {
    fn path(&self) -> String {
        self.loop_device.clone()
    }
}
impl Drop for MemBlk {
    fn drop(&mut self) {
        use std::time::Duration;
        
        let loop_device = &self.loop_device;
        let path = &self.path;
        run_cmd!(losetup -d $loop_device).unwrap();
        run_cmd!(rm $path).unwrap();
    }
}

const POOL_DIR: &str = "/mnt/dmtest";
pub struct MemBlkAllocator {
    i: u64,
}
impl MemBlkAllocator {
    pub fn new() -> Self {
        // -p: no error if existing
        run_cmd!(mkdir -p ${POOL_DIR}).unwrap();
        run_cmd!(mount -t ramfs -o size=2g ramfs ${POOL_DIR}).unwrap();
        Self {
            i: 0,
        }
    }
    fn alloc(&mut self) -> String {
        let path = format!("{}/{}", POOL_DIR, self.i);
        self.i += 1;
        path
    }
}
impl Drop for MemBlkAllocator {
    fn drop(&mut self) {
        run_cmd!(umount ${POOL_DIR}).unwrap();
        run_cmd!(rm -rf ${POOL_DIR}).unwrap();
    }
}