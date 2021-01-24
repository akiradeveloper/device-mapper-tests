use std::time::{Instant, Duration};
use crate::Sector;
use super::*;

pub enum Pattern {
    Skip(Sector),
    Write(Sector),
    Read(Sector),
}
pub struct PatternIO {
    cur: Option<Sector>,
    max_runtime: Option<Duration>,
    max_io_amount: Option<Sector>,
}
impl PatternIO {
    pub fn new() -> Self {
        Self {
            cur: None,
            max_runtime: None,
            max_io_amount: None,
        }
    }
    pub fn cur(mut self, x: Sector) -> Self {
        self.cur = Some(x);
        self
    }
    pub fn max_runtime(mut self, x: Duration) -> Self {
        self.max_runtime = Some(x);
        self
    }
    pub fn max_io_amount(mut self, x: Sector) -> Self {
        self.max_io_amount = Some(x);
        self
    }
    pub fn submit(&self, rw: &OpenState, pats: &[Pattern]) {
        let start = self.cur.unwrap_or(Sector(0));
        let mut cur = start;
        let deadline = self.max_runtime.map(|x| Instant::now() + x);
        let max_io_amount = self.max_io_amount.unwrap_or(Sector(u64::max_value()));

        let mut should_quit = false;
        while !should_quit {
            use Pattern::*;
            for pat in pats {
                let l = match pat {
                    Skip(l) => {
                        l
                    },
                    Write(l) => {
                        let buf = super::buf(&vec![0xFF; l.bytes() as usize]);
                        rw.write(&buf, cur, *l);
                        l
                    },
                    Read(l) => {
                        let mut buf = super::buf(&vec![0; l.bytes() as usize]);
                        rw.read(&mut buf, cur, *l);
                        l
                    }
                };
                cur = cur + *l;
            }
            if cur - start >= max_io_amount {
                should_quit = true;
            }
            if let Some(x) = deadline {
                if Instant::now() >= x {
                    should_quit = true;
                }
            }
        }
    }
}