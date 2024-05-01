// #![feature(specialization)]

pub mod blkdev;
pub mod dm;
mod env;
pub mod fs;
pub mod io;
pub mod kernel;
mod rand_name;
pub mod scenario;
mod sector;
pub mod stack;

pub use env::env;
pub use io::open;
use rand_name::rand_name;
pub use sector::Sector;

pub trait Stack {
    fn path(&self) -> String;
}
pub trait DMStack: Stack {
    fn dm(&self) -> &dm::State;
}
impl<T: DMStack> Stack for T {
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
    dm: dm::State,
}
impl DMStack for EmptyDMStack {
    fn dm(&self) -> &dm::State {
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
        let dm = dm::State::new(name);
        dm.create();
        EmptyDMStack { dm }
    }
}
pub trait DMStackDecorator: DMStack {
    fn delegate(&self) -> &dyn DMStack;
}
impl<T: DMStackDecorator> DMStack for T {
    fn dm(&self) -> &dm::State {
        self.delegate().dm()
    }
}
pub trait DMTable {
    fn line(&self) -> String;
}
