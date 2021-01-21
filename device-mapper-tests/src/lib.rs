// #![feature(specialization)]

mod sector;
mod env;
mod rand_name;
mod dm;
pub mod stack;
pub mod fs;
pub mod io;
pub mod kernel;
pub mod blkdev;

use std::time::Duration;
use dm::DMState;
use rand_name::rand_name;
pub use sector::Sector;
pub use env::env;
pub use io::open;

pub trait Stack {
    fn path(&self) -> String;
}
pub trait DMStack: Stack {
    fn dm(&self) -> &DMState;
}
impl <T: DMStack> Stack for T {
    fn path(&self) -> String {
        self.dm().path()
    }
}
pub fn reload(stack: impl DMStack, table: &impl DMTable) -> impl DMStack {
    stack.dm().suspend();
    stack.dm().reload(&table.line());
    stack.dm().resume();
    stack
}
pub struct EmptyDMStack {
    dm: DMState,
}
impl DMStack for EmptyDMStack {
    fn dm(&self) -> &DMState {
        &self.dm
    }
}
impl Drop for EmptyDMStack {
    fn drop(&mut self) {
        self.dm().remove()
    }
}
impl EmptyDMStack {
    pub fn new() -> Self {
        let name = rand_name();
        let dm = DMState::new(name);
        dm.create();
        EmptyDMStack {
            dm,
        }
    }
}
pub trait DMStackDecorator: DMStack {
    fn delegate(&self) -> &DMStack;
}
impl <T: DMStackDecorator> DMStack for T {
    fn dm(&self) -> &DMState {
        self.delegate().dm()
    }
}
pub trait DMTable {
    fn line(&self) -> String;
}