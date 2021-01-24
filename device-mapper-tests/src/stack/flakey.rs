use crate::{Sector, DMTable, DMStack, DMStackDecorator};
use std::time::Duration;

pub struct Table {
    pub backing_dev: String,
    pub up_interval_sec: u64,
    pub down_interval_sec: u64,
}
impl DMTable for Table {
    fn line(&self) -> String {
        let sz = crate::blkdev::get_size(&self.backing_dev).sectors();
        // Starting from the time the table is loaded, the device is available for <up interval> seconds,
        // then exhibits unreliable behaviour for <down interval> seconds, and then this cycle repeats.
        format!("0 {} flakey {} 0 {} {}", sz, self.backing_dev, self.up_interval_sec, self.down_interval_sec)
    }
}
pub struct Flakey {
    delegate: Box<DMStack>,
    table: Table
}
impl Flakey {
    pub fn new<S: DMStack + 'static>(s: S, table: Table) -> Self {
        let s = crate::reload(s, &table);
        Self {
            delegate: Box::new(s),
            table,
        }
    }
}
impl DMStackDecorator for Flakey {
    fn delegate(&self) -> &DMStack {
        self.delegate.as_ref()
    }
}