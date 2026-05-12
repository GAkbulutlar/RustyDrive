# 🤖 RustyDrive — ROS 2 Keyboard Teleop in Rust

RustyDrive is a keyboard-controlled ROS 2 teleoperation node written in Rust with [`rclrs`](https://github.com/ros2-rust/ros2_rust). It publishes `geometry_msgs/msg/Twist` messages to the `/cmd_vel` topic for robot velocity control.

## Features

- Keyboard-based teleoperation
- ROS 2 integration using `rclrs`
- Publishes velocity commands to `/cmd_vel`
- Memory-safe implementation in Rust
- Smooth velocity control and modular project structure

## Project Structure

```text
src/
├── main.rs                 # Entry point and ROS 2 node setup
├── lib.rs                  # Module exports
├── config.rs               # RobotConfig: speeds and topic settings
├── keyboard_input.rs       # Key input handling and help display
├── velocity_controller.rs  # Velocity ramping and clamping
├── motion_patterns.rs      # Optional predefined motion patterns
└── telemetry.rs            # Session statistics and runtime telemetry
```

## Prerequisites

| Tool | Recommended Version |
|------|----------------------|
| ROS 2 | Jazzy |
| Rust | stable (1.75+) |
| `rclrs` / `ros2_rust` | 0.7.0 |

Make sure your ROS 2 environment and Rust toolchain are installed and configured correctly.

## Build

```bash
source /opt/ros/jazzy/setup.bash
source ~/ros2_ws/install/setup.bash
cd ~/ros2_ws/src/RustyDrive
cargo build
```

## Run

```bash
./target/debug/robot_controller
```

## Controls

| Key | Action |
|-----|--------|
| `W` | Move forward |
| `S` | Move backward |
| `A` | Turn left |
| `D` | Turn right |
| `Space` | Emergency stop |
| `Q` | Quit and show session summary |

## Modules

### `config.rs`
Defines the `RobotConfig` structure, including speed limits and topic configuration.

### `velocity_controller.rs`
Gradually ramps velocity toward target values and clamps motion to configured limits for smoother control.

### `motion_patterns.rs`
Provides reusable autonomous motion patterns such as circle, square, figure-eight, and spin-in-place.

### `telemetry.rs`
Tracks session activity such as commands sent, estimated linear distance, total angular rotation, and uptime.

## Testing

```bash
cargo test
```

Unit tests are included in modules such as `velocity_controller.rs`, `motion_patterns.rs`, and `telemetry.rs`.
