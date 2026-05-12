# 🤖 RustyDrive — ROS 2 Robot Controller in Rust

A keyboard-driven ROS 2 robot controller written in Rust using [`rclrs`](https://github.com/ros2-rust/ros2_rust). Publishes `geometry_msgs/Twist` messages to `/cmd_vel` with smooth velocity ramping, predefined motion patterns, and live telemetry.

---

## 📦 Project Structure

src/
├── main.rs # Entry point — ROS 2 node setup & main loop
├── lib.rs # Module exports
├── config.rs # RobotConfig — speeds, topic name
├── keyboard_input.rs # Key → KeyCommand mapping + help display
├── velocity_controller.rs # Smooth velocity ramping with speed clamping
├── motion_patterns.rs # Predefined patterns: circle, square, figure-eight, spin
└── telemetry.rs # Session stats: distance, rotation, uptime


---

## 🚀 Prerequisites

| Tool | Version |
|------|---------|
| ROS 2 | Jazzy |
| Rust | stable (1.75+) |
| ros2_rust / rclrs | 0.7.0 |

---

## 🔧 Build

```bash
source /opt/ros/jazzy/setup.bash
source ~/ros2_ws/install/setup.bash
cd ~/ros2_ws/src/robot_controller
cargo build


▶️ Run

./target/debug/robot_controller


⌨️ Controls

Key	Action
W	Forward
S	Backward
A	Turn Left
D	Turn Right
Space	Emergency Stop
Q	Quit + show session summary

📐 Modules
config.rs
Centralised RobotConfig struct. Adjust speeds and the /cmd_vel topic here.

velocity_controller.rs
Ramps velocity toward the target in fixed steps and clamps to configured limits — preventing jerky motion.

motion_patterns.rs
Returns Vec<MotionStep> for autonomous patterns: circle, square, figure_eight, spin_in_place.

telemetry.rs
Tracks commands sent, estimated linear distance, and total angular rotation. Prints a summary table on quit.

🧪 Tests

cargo test

Unit tests are included in velocity_controller.rs, motion_patterns.rs, and telemetry.rs.


