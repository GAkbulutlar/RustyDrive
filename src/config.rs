/// Robot configuration parameters.
pub struct RobotConfig {
    pub linear_speed: f64,
    pub angular_speed: f64,
    pub max_linear_speed: f64,
    pub max_angular_speed: f64,
    pub cmd_vel_topic: &'static str,
}

impl Default for RobotConfig {
    fn default() -> Self {
        Self {
            linear_speed: 0.5,
            angular_speed: 0.5,
            max_linear_speed: 1.0,
            max_angular_speed: 1.5,
            cmd_vel_topic: "/cmd_vel",
        }
    }
}
