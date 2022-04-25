use crate::*;
pub fn run_bash(command: String) {
    println!("{} {} {}", "Running ".bright_yellow(), command.bright_blue(), "in bash".bright_red());
}