use std::time::{Duration, Instant};

pub struct Throttle {
    // settings
    duration: Duration,
    executions: usize,

    // inner data
    instant: Instant,
    count: usize
}

impl Throttle {
    /// Creates new Throttle
    pub(crate) fn new(duration: Duration, executions: usize) -> Self {
        return Self {
            duration,
            executions,
            instant: Instant::now(),
            count: 0usize
        }
    }
    
    /// Updates and returns true if valid throttle
    pub(crate) fn update(&mut self) -> bool {
        let now = Instant::now();

        if now.duration_since(self.instant) < self.duration {
            if self.count < self.executions {
                self.count+=1;
                true
            } else {
                false
            }
        } else {
            // reset
            self.instant = now;
            self.count = 0;
            true
        }
    }
}