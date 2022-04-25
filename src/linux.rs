use crate::*;

pub fn run_bash(command: String) {
    println!("{} {}", "Executing".bright_yellow(), command.bright_blue());
    let lines: Vec<&str> = command.split("\n").collect();
    for line in lines {
        if line.contains(" && ") || line.contains(" | ") || line.contains(" >> ") || line == "" {
            println!(
                "{}",
                "Some bash syntax is not supported skipping the line!".bright_red()
            )
        } else {
            let words: Vec<&str> = line.split_whitespace().collect();
            let cmd = words[0];
            let wrd = 0;
            let mut args: String = "".to_string();
            for word in words {
                if wrd == 0 && word == cmd {
                    continue;
                } else {
                    args = args + word;
                }
            }
            let out = std::process::Command::new(cmd)
                .arg(args)
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
