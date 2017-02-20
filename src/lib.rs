//! Timekeeper is a simple library to track the amount of time used by different
//! parts of a program.

use std::time;

pub mod simpletimer;

pub trait Timer {
    /// Start timing. Panics if the timer is already running.
    fn start(&mut self);

    /// Stop timing. Panics if the timer is not currently running.
    fn stop(&mut self);

    /// Get the elapsed time of this timer.
    fn get(&self) -> time::Duration;

    /// Same as `get()` but returns the elapsed time as a number of nanoseconds
    /// instead of a Duration. Panics on overflow.
    fn num_nanoseconds(&self) -> u64 {
        let dur = self.get();
        dur.as_secs().checked_mul(1000_000_000).unwrap() + (dur.subsec_nanos() as u64)
    }

    /// Same as `get()` but returns the elapsed time as a number of microseconds
    /// instead of a Duration. Panics on overflow.
    fn num_microseconds(&self) -> u64 {
        let dur = self.get();
        dur.as_secs().checked_mul(1000_000).unwrap() + (dur.subsec_nanos() as u64 / 1000)
    }

    /// Same as `get()` but returns the elapsed time as a number of milliseconds
    /// instead of a Duration. Panics on overflow.
    fn num_milliseconds(&self) -> u64 {
        let dur = self.get();
        dur.as_secs().checked_mul(1000).unwrap() + (dur.subsec_nanos() as u64 / 1000_000)
    }

    /// Same as `get()` but returns the elapsed time as a number of seconds
    /// instead of a Duration.
    fn num_seconds(&self) -> u64 {
        self.get().as_secs()
    }

    /// Same as `get()` but returns the elapsed time as a number of minutes
    /// instead of a Duration.
    fn num_minutes(&self) -> u64 {
        self.get().as_secs() / 60
    }

    /// Same as `get()` but returns the elapsed time as a number of hours
    /// instead of a Duration.
    fn num_hours(&self) -> u64 {
        self.get().as_secs() / 3600
    }
}