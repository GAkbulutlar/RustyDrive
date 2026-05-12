use geometry_msgs::msg::Twist;

/// Smoothly ramps velocity toward a target, enforcing speed limits.
pub struct VelocityController {
    max_linear:      f64,
    max_angular:     f64,
    current_linear:  f64,
    current_angular: f64,
    ramp_step:       f64,
}

impl VelocityController {
    pub fn new(max_linear: f64, max_angular: f64) -> Self {
        Self {
            max_linear,
            max_angular,
            current_linear:  0.0,
            current_angular: 0.0,
            ramp_step:       0.05,
        }
    }

    /// Advance current velocity one step toward `target` and return the
    /// smoothed Twist that should be published.
    pub fn apply(&mut self, target: &Twist) -> Twist {
        self.current_linear = ramp(self.current_linear, target.linear.x, self.ramp_step)
            .clamp(-self.max_linear, self.max_linear);
        self.current_angular = ramp(self.current_angular, target.angular.z, self.ramp_step)
            .clamp(-self.max_angular, self.max_angular);

        let mut out = Twist::default();
        out.linear.x  = self.current_linear;
        out.angular.z = self.current_angular;
        out
    }

    /// Immediately zero velocity and return a stop Twist.
    pub fn stop(&mut self) -> Twist {
        self.current_linear  = 0.0;
        self.current_angular = 0.0;
        Twist::default()
    }

    pub fn current_linear(&self)  -> f64 { self.current_linear  }
    pub fn current_angular(&self) -> f64 { self.current_angular }
}

fn ramp(current: f64, target: f64, step: f64) -> f64 {
    let delta = target - current;
    if delta.abs() <= step { target } else { current + step * delta.signum() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ramps_up_toward_target() {
        let mut vc = VelocityController::new(1.0, 1.0);
        let mut t = Twist::default();
        t.linear.x = 1.0;
        let out = vc.apply(&t);
        assert!(out.linear.x > 0.0 && out.linear.x <= 1.0);
    }

    #[test]
    fn stop_zeros_velocity() {
        let mut vc = VelocityController::new(1.0, 1.0);
        let mut t = Twist::default();
        t.linear.x = 1.0;
        for _ in 0..30 { vc.apply(&t); }
        vc.stop();
        assert_eq!(vc.current_linear(), 0.0);
    }

    #[test]
    fn clamps_to_max_speed() {
        let mut vc = VelocityController::new(0.3, 0.3);
        let mut t = Twist::default();
        t.linear.x = 999.0;
        for _ in 0..200 { vc.apply(&t); }
        assert!(vc.current_linear() <= 0.3 + f64::EPSILON);
    }
}
