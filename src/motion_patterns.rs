use geometry_msgs::msg::Twist;
use std::time::Duration;

/// A single timed motion command.
pub struct MotionStep {
    pub twist:    Twist,
    pub duration: Duration,
}

/// Drive a circle of `radius` metres at `speed` m/s for one full loop.
pub fn circle(speed: f64, radius: f64) -> Vec<MotionStep> {
    let angular      = if radius > 0.0 { speed / radius } else { 0.0 };
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let secs          = (circumference / speed.abs()).ceil() as u64;
    let mut t = Twist::default();
    t.linear.x  = speed;
    t.angular.z = angular;
    vec![MotionStep { twist: t, duration: Duration::from_secs(secs) }]
}

/// Drive a square; each straight side lasts `side_secs` seconds.
pub fn square(speed: f64, side_secs: u64) -> Vec<MotionStep> {
    let mut steps = Vec::new();
    for _ in 0..4 {
        let mut fwd = Twist::default();
        fwd.linear.x = speed;
        steps.push(MotionStep { twist: fwd, duration: Duration::from_secs(side_secs) });

        let mut turn = Twist::default();
        turn.angular.z = std::f64::consts::FRAC_PI_2;
        steps.push(MotionStep { twist: turn, duration: Duration::from_secs(1) });
    }
    steps
}

/// Drive a figure-eight (two circles in opposite directions).
pub fn figure_eight(speed: f64, radius: f64) -> Vec<MotionStep> {
    let angular   = speed / radius;
    let half_secs = (std::f64::consts::PI * radius / speed).ceil() as u64 * 2;

    let mut left = Twist::default();
    left.linear.x  = speed;
    left.angular.z = angular;

    let mut right = Twist::default();
    right.linear.x  = speed;
    right.angular.z = -angular;

    vec![
        MotionStep { twist: left,  duration: Duration::from_secs(half_secs) },
        MotionStep { twist: right, duration: Duration::from_secs(half_secs) },
    ]
}

/// Spin in place for a full 360°.
pub fn spin_in_place(angular_speed: f64) -> Vec<MotionStep> {
    let secs = (2.0 * std::f64::consts::PI / angular_speed.abs()).ceil() as u64;
    let mut t = Twist::default();
    t.angular.z = angular_speed;
    vec![MotionStep { twist: t, duration: Duration::from_secs(secs) }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_has_eight_steps() {
        assert_eq!(square(0.5, 2).len(), 8);
    }

    #[test]
    fn figure_eight_has_two_steps() {
        assert_eq!(figure_eight(0.5, 0.5).len(), 2);
    }

    #[test]
    fn circle_angular_correct() {
        let steps = circle(1.0, 2.0);
        assert!((steps[0].twist.angular.z - 0.5).abs() < 1e-9);
    }
}
