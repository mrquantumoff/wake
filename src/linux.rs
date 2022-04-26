use crate::*;

pub fn run_bash(command: String, _debug: String) {
    let lines: Vec<&str> = command.split("\n").collect();
    for line in lines {
        if line.contains(" && ") || line.contains(" | ") || line.contains(" >> ") {
            println!(
                "{}",
                "Some bash syntax is not supported skipping the line!".bright_red()
            )
        } else if line == "" {
        } else {
            println!("{} {}", "Executing".bright_yellow(), line.bright_blue());
            let out = std::process::Command::new("bash")
                .arg("-c")
                .arg(line)
                .status()
                .expect("The program failed");
            if out.success() {
                println!(
                    "{} {}",
                    "The program finished with".bright_green(),
                    out.to_string().bright_green()
                )
            } else {
                println!(
                    "{}, {}",
                    "The program failed with exit code ".bright_red(),
                    out
                )
            }
        }
    }
}
