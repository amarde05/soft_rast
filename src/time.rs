use std::time::Duration;

use chrono::{DateTime, TimeDelta, Utc};


pub struct Time {
    dt: f32,
    last_update: DateTime<Utc>
}

impl Time {
    pub fn new() -> Self {
        Time {
            dt: 0.,
            last_update: Utc::now()
        }
    }

    pub fn tick(&mut self) -> f32{
        let now = Utc::now();
        self.dt = (now - self.last_update).as_seconds_f32();
        self.last_update = now;
        self.dt
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }
}