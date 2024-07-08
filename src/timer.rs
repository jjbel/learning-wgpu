use std::time::{Duration, Instant};

pub struct Timer {
    pub instant: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            instant: Instant::now(),
        }
    }

    pub fn duration(&self) -> Duration {
        self.instant.elapsed()
    }

    pub fn reset(&mut self) {
        self.instant = Instant::now();
    }

    pub fn millis(&self) -> f32 {
        (self.duration().as_nanos() as f32) / 1_000_000.0
    }

    pub fn str(&self) -> String {
        format!("{:8.4}ms", self.millis())
    }

    pub fn str_reset(&mut self) -> String {
        let str = self.str();
        self.reset();
        str
    }

    pub fn delay_until(&self, total_duration: Duration) {
        
    }
}
