// #![feature(specialization)]

use crate::Stack;

pub mod flakey;
pub mod linear;
pub mod luks;

struct RawBlk {
    path: String,
}
impl Stack for RawBlk {
    fn path(&self) -> String {
        self.path.clone()
    }
}
impl Drop for RawBlk {
    fn drop(&mut self) {}
}
