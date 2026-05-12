use std::time::{Duration, Instant};

/// Tracks runtime statistics for a controller session.
pub struct Telemetry {
    start_time:             Instant,
    pub commands_sent:      u64,
    total_linear_distance:  f64,
    total_angular_rotation: f64,
}

impl Default for Telemetry {
    fn default() -> Self { Self::new() }
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            start_time:             Instant::now(),
            commands_sent:          0,
            total_linear_distance:  0.0,
            total_angular_rotation: 0.0,
        }
    }

    /// Call after each publish with the velocities sent and elapsed time.
    pub fn record(&mut self, linear_x: f64, angular_z: f64, dt: Duration) {
        self.commands_sent          += 1;
        self.total_linear_distance  += linear_x.abs()  * dt.as_secs_f64();
        self.total_angular_rotation += angular_z.abs() * dt.as_secs_f64();
    }

    pub fn uptime(&self)             -> Duration { self.start_time.elapsed() }
    pub fn estimated_distance(&self) -> f64      { self.total_linear_distance  }
    pub fn estimated_rotation(&self) -> f64      { self.total_angular_rotation }

    pub fn print_summary(&self) {
        println!("┌──────────────────────────────┐");
        println!("│       Session Telemetry      │");
        println!("├──────────────────────────────┤");
        println!("│  Uptime:        {:>8.1} s   │", self.uptime().as_secs_f64());
        println!("│  Commands sent: {:>8}     │", self.commands_sent);
        println!("│  Est. distance: {:>8.2} m   │", self.total_linear_distance);
        println!("│  Est. rotation: {:>8.2} rad │", self.total_angular_rotation);
        println!("└──────────────────────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_distance() {
        let mut t = Telemetry::new();
        t.record(1.0, 0.0, Duration::from_secs(2));
        assert!((t.estimated_distance() - 2.0).abs() < 1e-9);
        assert_eq!(t.commands_sent, 1);
    }

    #[test]
    fn records_rotation() {
        let mut t = Telemetry::new();
        t.record(0.0, 1.0, Duration::from_secs(3));
        assert!((t.estimated_rotation() - 3.0).abs() < 1e-9);
    }
}
