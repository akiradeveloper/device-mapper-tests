use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn rand_name() -> String {
    format!("dmtest-{}", COUNTER.fetch_add(1, Ordering::SeqCst))
}
