use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Time {
    startup_time: Instant,
    last_update: Instant,
    delta: Duration,
    elapsed: Duration,
    time_scale: f32,
    frame_count: u64,
}

impl Default for Time {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            startup_time: now,
            last_update: now,
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            time_scale: 1.0,
            frame_count: 0,
        }
    }
}

impl Time {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_update);
        self.elapsed = now.duration_since(self.startup_time);
        self.last_update = now;
        self.frame_count += 1;
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32() * self.time_scale
    }

    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta.as_secs_f64() * self.time_scale as f64
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    pub fn elapsed_seconds_f64(&self) -> f64 {
        self.elapsed.as_secs_f64()
    }

    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn fps(&self) -> f32 {
        if self.delta.as_secs_f32() > 0.0 {
            1.0 / self.delta.as_secs_f32()
        } else {
            0.0
        }
    }
}