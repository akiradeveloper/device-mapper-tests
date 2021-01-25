use device_mapper_tests::*;

use std::collections::{HashMap, VecDeque};
use cmd_lib::run_cmd;

pub fn sweep_caches(cache_dev: &impl Stack) {
    let path = cache_dev.path();
    run_cmd!(dd status=none if=/dev/zero of=$path oflag=direct bs=512 count=1);
}
#[derive(Default)]
pub struct Options {
    m: HashMap<&'static str, u64>,
}
impl Options {
    pub fn writeback_threshold(mut self, x: u64) -> Self {
        assert!((0..=100).contains(&x));
        self.m.insert("writeback_threshold", x);
        self
    }
    pub fn nr_max_batched_writeback(mut self, x: u64) -> Self {
        assert!((1..=32).contains(&x));
        self.m.insert("nr_max_batched_writeback", x);
        self
    }
    pub fn update_sb_record_interval(mut self, x: u64) -> Self {
        assert!((0..=3600).contains(&x));
        self.m.insert("update_sb_record_interval", x);
        self
    }
    pub fn sync_data_interval(mut self, x: u64) -> Self {
        assert!((0..=3600).contains(&x));
        self.m.insert("sync_data_interval", x);
        self
    }
    pub fn read_cache_threshold(mut self, x: u64) -> Self {
        assert!((0..=127).contains(&x));
        self.m.insert("read_cache_threshold", x);
        self
    }
    pub fn write_around_mode(mut self) -> Self {
        self.m.insert("write_around_mode", 1);
        self
    }
    pub fn nr_read_cache_cells(mut self, x: u64) -> Self {
        assert!((1..=2048).contains(&x));
        self.m.insert("nr_read_cache_cells", x);
        self
    }
    fn mk_line_repr(&self) -> String {
        let n = self.m.len();
        if n == 0 {
            "".to_owned()
        } else {
            let mut s = format!(" {}", n*2);
            for (k, v) in &self.m {
                let kv = format!(" {} {}", k, v);
                s.push_str(&kv);
            }
            dbg!(&s);
            s
        }
    }
}
pub struct Table {
    pub backing_dev: String,
    pub cache_dev: String,
    pub options: Options,
}
impl DMTable for Table {
    fn line(&self) -> String {
        let sz = blkdev::get_size(&self.backing_dev).sectors();
        let backing_dev = &self.backing_dev;
        let cache_dev = &self.cache_dev;
        let option_line = self.options.mk_line_repr();
        format!("0 {} writeboost {} {}{}", sz, backing_dev, cache_dev, option_line)
    }
}
#[derive(Hash, Eq)]
pub struct StatKey {
    pub write: bool,
    pub hit: bool,
    pub on_buffer: bool,
    pub full_size: bool,
}
impl PartialEq for StatKey {
    fn eq(&self, that: &Self) -> bool {
        let x = (self.write, self.hit, self.on_buffer, self.full_size);
        let y = (that.write, that.hit, that.on_buffer, that.full_size);
        x == y
    }
}
impl StatKey {
    pub fn from_bits(mask: u8) -> Self {
        let on = |shift: u8| {
            (mask & (1<<shift)) != 0
        };
        StatKey {
            write: on(3),
            hit: on(2),
            on_buffer: on(1),
            full_size: on(0),
        }
    }
}
pub struct Status {
    pub cursor_pos: u64,
    pub nr_cache_blocks: u64,
    pub nr_segments: u64,
    pub current_id: u64,
    pub last_flushed_id: u64,
    pub last_writeback_id: u64,
    pub nr_dirty_cache_blocks: u64,
    pub stat: HashMap<StatKey, u64>,
    pub nr_partial_flushed: u64,
    pub options: HashMap<String, u64>,
}
fn pop_int(q: &mut VecDeque<String>) -> u64 {
    use std::str::FromStr;

    let x = q.pop_front().unwrap();
    u64::from_str(&x).unwrap()
}
impl Status {
    fn from(dm_status: dm::Status) -> Self {
        let mut q = VecDeque::new();
        for x in dm_status.args {
            q.push_back(x);
        }
        Status {
            cursor_pos: pop_int(&mut q),
            nr_cache_blocks: pop_int(&mut q),
            nr_segments: pop_int(&mut q),
            current_id: pop_int(&mut q),
            last_flushed_id: pop_int(&mut q),
            last_writeback_id: pop_int(&mut q),
            nr_dirty_cache_blocks: pop_int(&mut q),
            stat: {
                let mut m = HashMap::new();
                for i in 0..16 {
                    let k = StatKey::from_bits(i);
                    let v = pop_int(&mut q);
                    m.insert(k, v);
                }
                m
            },
            nr_partial_flushed: pop_int(&mut q),
            options: {
                q.pop_front(); // discard nr_tunables
                let mut m = HashMap::new();
                while !q.is_empty() {
                    let k = q.pop_front().unwrap();
                    let v = pop_int(&mut q);
                    m.insert(k, v);
                }
                m
            },
        }
    }
}
pub struct Writeboost {
    delegate: Box<DMStack>,
    table: Table,
}
impl Writeboost {
    pub fn create(table: Table) -> Self {
        let dm = EmptyDMStack::new();
        Self::new(dm, table)
    }
    pub fn new<S: DMStack + 'static>(s: S, table: Table) -> Self {
        let s = crate::reload(s, &table);
        Self {
            delegate: Box::new(s),
            table,
        }
    }
}
impl DMStackDecorator for Writeboost {
    fn delegate(&self) -> &DMStack {
        self.delegate.as_ref()
    }
}
impl Writeboost {
    pub fn drop_caches(&self) {
        self.dm().message("drop_caches");
    }
    pub fn clear_stat(&self) {
        self.dm().message("clear_stat");
    }
    pub fn status(&self) -> Status {
        Status::from(self.dm().status())
    }
}