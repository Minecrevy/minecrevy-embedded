use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    sync::OnceLock,
    task::Waker,
    time::{Duration, Instant},
};

use crossbeam_channel::Sender;
use embassy_time_driver::{Driver, time_driver_impl};

time_driver_impl!(static DRIVER: StdTimeDriver = StdTimeDriver::new());

pub struct StdTimeDriver {
    inner: OnceLock<DriverImpl>,
}

impl StdTimeDriver {
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }
}

impl Default for StdTimeDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for StdTimeDriver {
    fn now(&self) -> u64 {
        let inner = self.inner.get_or_init(DriverImpl::new);
        inner.start.elapsed().as_micros() as u64
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        let now = self.now();
        let inner = self.inner.get_or_init(DriverImpl::new);
        if at <= now {
            waker.wake_by_ref();
        } else {
            inner
                .sender
                .send(ScheduledWake {
                    at: inner.start + Duration::from_micros(at),
                    waker: waker.clone(),
                })
                .unwrap();
        }
    }
}

struct DriverImpl {
    start: Instant,
    sender: Sender<ScheduledWake>,
}

impl DriverImpl {
    fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded::<ScheduledWake>();

        std::thread::spawn(move || {
            let mut wake_queue = BinaryHeap::new(); // Min-heap for wake times
            loop {
                // Process incoming scheduled wakes
                let mut disconnected = false;
                loop {
                    match receiver.try_recv() {
                        Ok(scheduled) => wake_queue.push(Reverse(scheduled)),
                        Err(crossbeam_channel::TryRecvError::Empty) => break,
                        Err(crossbeam_channel::TryRecvError::Disconnected) => {
                            disconnected = true;
                            break;
                        }
                    }
                }

                // Check the next wake-up time
                if let Some(Reverse(scheduled)) = wake_queue.peek() {
                    let now = Instant::now();
                    if now >= scheduled.at {
                        // Wake the task
                        if let Some(Reverse(scheduled)) = wake_queue.pop() {
                            scheduled.waker.wake();
                        }
                    } else {
                        spin_sleep::sleep_until(scheduled.at);
                    }
                } else {
                    // No scheduled wakes, yield the thread
                    std::thread::yield_now();
                }

                if wake_queue.is_empty() && disconnected {
                    // No more scheduled wakes and our handle got dropped, exit the thread
                    return;
                }
            }
        });

        Self {
            start: Instant::now(),
            sender,
        }
    }
}

impl Default for DriverImpl {
    fn default() -> Self {
        Self::new()
    }
}

struct ScheduledWake {
    at: Instant,
    waker: Waker,
}

impl PartialEq for ScheduledWake {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at
    }
}

impl Eq for ScheduledWake {}

impl PartialOrd for ScheduledWake {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledWake {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.at.cmp(&other.at)
    }
}
