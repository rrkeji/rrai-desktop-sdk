use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::{
    sync::Notify,
    task::{spawn, JoinHandle},
    time::{sleep, Duration},
};

#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<RateLimiterInner>,
    #[allow(dead_code)]
    rate_task: Arc<JoinHandle<()>>,
}

impl RateLimiter {
    #[must_use]
    pub fn new(max_per_unit_time: usize, unit_time_ms: usize) -> Self {
        let inner = Arc::new(RateLimiterInner::new(max_per_unit_time, unit_time_ms));
        let rate_task = Arc::new({
            let inner = inner.clone();
            spawn(async move {
                inner.check_reset().await;
            })
        });
        Self { inner, rate_task }
    }

    pub async fn acquire(&self) {
        self.inner.acquire().await;
    }
}

struct RateLimiterInner {
    max_per_unit_time: usize,
    unit_time_ms: usize,
    remaining: AtomicUsize,
    notify: Notify,
}

impl RateLimiterInner {
    fn new(max_per_unit_time: usize, unit_time_ms: usize) -> Self {
        Self {
            max_per_unit_time,
            unit_time_ms,
            remaining: AtomicUsize::new(max_per_unit_time),
            notify: Notify::new(),
        }
    }

    fn decrement_remaining(&self) -> bool {
        fn gtzero(x: usize) -> Option<usize> {
            if x > 0 {
                Some(x - 1)
            } else {
                None
            }
        }

        self.remaining
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, gtzero)
            .is_ok()
    }

    async fn acquire(&self) {
        loop {
            if self.decrement_remaining() {
                return;
            }
            self.notify.notified().await;
        }
    }

    async fn check_reset(&self) {
        loop {
            self.remaining
                .fetch_max(self.max_per_unit_time, Ordering::SeqCst);
            self.notify.notify_waiters();
            sleep(Duration::from_millis(self.unit_time_ms as u64)).await;
        }
    }
}
 