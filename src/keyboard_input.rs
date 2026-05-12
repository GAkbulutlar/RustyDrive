use geometry_msgs::msg::Twist;
use crate::config::RobotConfig;

pub enum KeyCommand {
    Move(Twist),
    Stop,
    Quit,
    Unknown,
}

/// Map a single ASCII character to a robot command.
pub fn key_to_command(key: char, cfg: &RobotConfig) -> KeyCommand {
    let mut msg = Twist::default();
    match key {
        'w' | 'W' => { msg.linear.x = cfg.linear_speed;   KeyCommand::Move(msg) }
        's' | 'S' => { msg.linear.x = -cfg.linear_speed;  KeyCommand::Move(msg) }
        'a' | 'A' => { msg.angular.z = cfg.angular_speed;  KeyCommand::Move(msg) }
        'd' | 'D' => { msg.angular.z = -cfg.angular_speed; KeyCommand::Move(msg) }
        ' '       => KeyCommand::Stop,
        'q' | 'Q' => KeyCommand::Quit,
        _         => KeyCommand::Unknown,
    }
}

pub fn print_help() {
    println!("┌──────────────────────────────┐");
    println!("│   Robot Controller Controls  │");
    println!("├──────────────────────────────┤");
    println!("│  W / S   Forward / Backward  │");
    println!("│  A / D   Turn Left / Right   │");
    println!("│  SPACE   Emergency Stop      │");
    println!("│  Q       Quit & show stats   │");
    println!("└──────────────────────────────┘");
}
