extern crate chrono;
use std::time::SystemTime;
use chrono::Local;

/// Tracks the time for a process
#[derive(Debug)]
pub struct  Clock {
    /// Start time for the clock
    pub start: i32,
    /// Time the process is at currently
    pub step: i32
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            start: 0,
            step: 0
        }
    }

    /// Gets the current time in the default format
    pub fn now() -> i32 {
        0
    }

    /// Gets the current time in the given format. Format options:
    /// https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
    pub fn format_now(format: &str) -> String {
        Local::now().format(format).to_string()
    }

    /// Increments the clock by one
    pub fn increment(&mut self) {
        self.step += 1;
    }

    /// Resets the clock back to the start
    pub fn reset(&mut self) {
        self.step = self.start;
    }
}