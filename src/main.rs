use anyhow::{Error, Result};
use rclrs::*;
use geometry_msgs::msg::Twist;
use std::io::Read;
use std::time::Duration;

use robot_controller::{
    config::RobotConfig,
    keyboard_input::{key_to_command, print_help, KeyCommand},
    velocity_controller::VelocityController,
    telemetry::Telemetry,
};

fn main() -> Result<(), Error> {
    let cfg = RobotConfig::default();

    let context  = Context::default_from_env()?;
    let executor = context.create_basic_executor();
    let node     = executor.create_node("robot_controller")?;
    let publisher = node.create_publisher::<Twist>(cfg.cmd_vel_topic)?;

    let mut vel_ctrl = VelocityController::new(cfg.max_linear_speed, cfg.max_angular_speed);
    let mut telem    = Telemetry::new();

    println!("Robot Controller Ready!");
    print_help();

    while context.ok() {
        let mut input = [0u8; 1];
        if std::io::stdin().read_exact(&mut input).is_err() {
            break;
        }

        match key_to_command(input[0] as char, &cfg) {
            KeyCommand::Move(target) => {
                let smoothed = vel_ctrl.apply(&target);
                publisher.publish(&smoothed)?;
                telem.record(smoothed.linear.x, smoothed.angular.z, Duration::from_millis(100));
                println!("  linear={:+.2}  angular={:+.2}", smoothed.linear.x, smoothed.angular.z);
            }
            KeyCommand::Stop => {
                let stop = vel_ctrl.stop();
                publisher.publish(&stop)?;
                println!("  [STOP]");
            }
            KeyCommand::Quit => {
                let stop = vel_ctrl.stop();
                publisher.publish(&stop)?;
                telem.print_summary();
                break;
            }
            KeyCommand::Unknown => {}
        }
    }
    Ok(())
}
