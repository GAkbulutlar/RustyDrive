use anyhow::{Error, Result};
use rclrs::*;
use geometry_msgs::msg::Twist;
use std::io::Read;

fn main() -> Result<(), Error> {
    let context = Context::default_from_env()?;
    let executor = context.create_basic_executor();
    let node = executor.create_node("robot_controller")?;
    let publisher = node.create_publisher::<Twist>("/cmd_vel")?;

    println!("Robot Controller Ready!");
    println!("W/S = forward/back | A/D = turn left/right | Q = quit");

    while context.ok() {
        let mut input = [0u8; 1];
        std::io::stdin().read_exact(&mut input).unwrap();

        let mut msg = Twist::default();
        match input[0] as char {
            'w' => msg.linear.x = 0.5,
            's' => msg.linear.x = -0.5,
            'a' => msg.angular.z = 0.5,
            'd' => msg.angular.z = -0.5,
            'q' => break,
            _   => {}
        }

        publisher.publish(&msg)?;
        println!("Published: linear={} angular={}", msg.linear.x, msg.angular.z);
    }
    Ok(())
}
