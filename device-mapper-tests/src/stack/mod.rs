// #![feature(specialization)]

use crate::{DMTable, Stack, DMStack, DMState, DMStackDecorator};
use crate::Sector;
use std::path::PathBuf;

pub mod linear;
pub mod flakey;
pub mod writeboost;
pub mod luks;

struct RawBlk {
    path: String
}
impl Stack for RawBlk {
    fn path(&self) -> String {
        self.path.clone()
    }
}
impl Drop for RawBlk {
    fn drop(&mut self) {}
}