use crate::{Sector, Stack};

mod mem_blk;
mod pool;

use mem_blk::{MemBlk, MemBlkAllocator};
use pool::DevicePool;

pub struct Env {
    pool: DevicePool,
    backing: MemBlk,
    blk_alloc: MemBlkAllocator,
}
impl Env {
    fn new() -> Self {
        let mut blk_alloc = MemBlkAllocator::new();
        let backing = MemBlk::new(&mut blk_alloc, 2 << 10);
        let pool = DevicePool::new(&backing.path());
        Self {
            pool,
            backing,
            blk_alloc,
        }
    }
    /// Allocate block device from memory.
    /// The total amount of memory is limited to 2GB.
    pub fn alloc_device(&mut self, len: Sector) -> impl Stack {
        self.pool.acquire(len)
    }
}
pub fn env() -> Env {
    Env::new()
}
