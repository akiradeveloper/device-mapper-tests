use super::*;
use crate::Stack;
use cmd_lib::run_cmd;

impl XFS {
    pub fn format(s: &impl Stack) {
        let path = s.path();
        run_cmd!(mkfs.xfs -f -m crc=1 $path).unwrap();
    }
}
pub struct XFS {
    backing_dev: String,
    pub mount_point: String,
}
impl XFS {
    pub fn new(s: &impl Stack) -> Self {
        let blk_dev = s.path();
        let mount_point = mount(s, "");
        Self {
            mount_point,
            backing_dev: blk_dev,
        }
    }
}
impl Drop for XFS {
    fn drop(&mut self) {
        umount(&self.mount_point);
        let blk_dev = &self.backing_dev;
        run_cmd!(xfs_repair -n $blk_dev).unwrap();
    }
}
