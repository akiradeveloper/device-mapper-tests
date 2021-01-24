use crate::{Stack, Sector, DMStack, EmptyDMStack, blkdev};
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Ord, Eq)]
struct Range {
    pub start: Sector,
    end: Sector,
}
impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.start.cmp(&other.start))
    }
}
impl Range {
    fn split(self, len: Sector) -> (Range, Range) {
        let a = Range { start: self.start, end: self.start+len };
        let b = Range { start: a.end, end: self.end };
        (a,b)
    }
    fn len(&self) -> Sector {
        self.end - self.start
    }
}
struct Pool {
    m: BTreeSet<Range>,
}
impl Pool {
    fn new(start: Sector, len: Sector) -> Self {
        let mut m = BTreeSet::new();
        m.insert(Range { start, end: start+len });
        Self {
            m
        }
    }
    fn acquire(&mut self, len: Sector) -> Option<Range> {
        let mut ret = None;
        for x in &self.m {
            if x.len().sectors() >= len.sectors() {
                ret = Some(*x)
            }
        }
        if let Some(x) = ret {
            self.m.remove(&x);
            let (a,b) = x.split(len);
            if b.len() > Sector(0) {
                self.m.insert(b);
            }
            Some(a)
        } else {
            None
        }
    }
    fn release(&mut self, x: Range) {
        self.m.insert(x);
        let mut xs = vec![];
        for &x in &self.m {
            xs.push(x);
        }
        let mut new_m = BTreeSet::new();
        for x in merge(xs) {
            new_m.insert(x);
        }
        self.m = new_m;
    }
}
fn merge(mut xs: Vec<Range>) -> Vec<Range> {
    xs.sort();
    loop {
        let mut next = false;

        let n = xs.len();
        for i in 0..n-1 {
            let j = i+1;
            let a = xs[i];
            let b = xs[j];
            if a.end == b.start {
                let mut new_xs = vec![];
                for k in 0..i {
                    new_xs.push(xs[k]);
                }
                new_xs.push(Range { start: a.start, end: b.end });
                for k in j+1..n {
                    new_xs.push(xs[k]);
                }
                xs = new_xs;
                next = true;
                break;
            }
        }

        if !next {
            break
        }
    }
    xs
}
#[test]
fn test_pool_1() {
    let mut pool = Pool::new(Sector(0), Sector(100));
    assert_eq!(pool.m.len(), 1);
    let x0 = pool.acquire(Sector(50)).unwrap();
    assert_eq!(pool.m.len(), 1);
    let x1 = pool.acquire(Sector(10)).unwrap();
    assert_eq!(pool.m.len(), 1);
    let x2 = pool.acquire(Sector(10)).unwrap();
    assert_eq!(pool.m.len(), 1);
    pool.release(x1);
    assert_eq!(pool.m.len(), 2);
    pool.release(x0);
    assert_eq!(pool.m.len(), 2);
    pool.release(x2);
    assert_eq!(pool.m.len(), 1);
}
#[test]
fn test_pool_2() {
    let mut pool = Pool::new(Sector(0), Sector(100));
    assert!(pool.acquire(Sector(101)).is_none());
    let x1 = pool.acquire(Sector(100)).unwrap();
    assert_eq!(pool.m.len(), 0);
    pool.release(x1);
    assert_eq!(pool.m.len(), 1);
}

pub struct DevicePool {
    backing_dev: String,
    pool: Arc<Mutex<Pool>>,
}
impl DevicePool {
    pub fn new(path: &str) -> Self {
        let sz = blkdev::get_size(&path);
        Self {
            backing_dev: path.to_owned(),
            pool: Arc::new(Mutex::new(Pool::new(Sector::GB(0), sz)))
        }
    }
    pub fn acquire(&mut self, len: Sector) -> impl Stack {
        let dm = EmptyDMStack::new();
        let range = self.pool.lock().unwrap().acquire(len).unwrap();
        let table = crate::stack::linear::Table {
            backing_dev: self.backing_dev.clone(),
            offset: range.start,
            len,
        };
        let dm = crate::reload(dm, &table);
        PoolLinear { dm: Box::new(dm), range, pool: Arc::clone(&self.pool) }
    }
}
pub struct PoolLinear {
    dm: Box<DMStack>,
    range: Range,
    pool: Arc<Mutex<Pool>>,
}
impl Stack for PoolLinear {
    fn path(&self) -> String {
        self.dm.path()
    }
}
impl Drop for PoolLinear {
    fn drop(&mut self) {
        self.pool.lock().unwrap().release(self.range)
    }
}