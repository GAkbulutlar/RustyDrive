# Robot Controller - ROS 2 Rust Edition

A simple keyboard-controlled ROS 2 robot controller written in Rust. This package provides a command-line interface for controlling a robot's velocity through keyboard input.

## Features

- **Keyboard Control**: Intuitive WASD controls for robot movement
- **ROS 2 Integration**: Uses rclrs for ROS 2 communication
- **Twist Messages**: Publishes velocity commands to `/cmd_vel` topic
- **Rust Safety**: Leverages Rust's memory and thread safety guarantees

## Controls

| Key | Action |
|-----|--------|
| **W** | Move forward (linear velocity: 0.5 m/s) |
| **S** | Move backward (linear velocity: -0.5 m/s) |
| **A** | Turn left (angular velocity: 0.5 rad/s) |
| **D** | Turn right (angular velocity: -0.5 rad/s) |
| **Q** | Quit the controller |

## Prerequisites

- ROS 2 (Humble or later recommended)
- Rust toolchain (1.70+)
- rclrs bindings
- geometry_msgs ROS 2 package

## Building

```bash
colcon build --packages-select robot_controller
