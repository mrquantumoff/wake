use crate::*;

pub fn run_bash(command: String, debug: String) {
    
    let lines: Vec<&str> = command.split("\n").collect();
    for line in lines {
        if line.contains(" && ") || line.contains(" | ") || line.contains(" >> ") {
            println!(
                "{}",
                "Some bash syntax is not supported skipping the line!".bright_red()
            )
        }
        else if line == "" {
            
        } 
        else {
            println!("{} {}", "Executing".bright_yellow(), line.bright_blue());
            let words: Vec<&str> = line.split_whitespace().collect();
            #[allow(unused_assignments)]
            let mut cmd = "";
            if words.len() == 0 {
                cmd = line;
            }
            else {
                cmd = words[0];
            }
            let wrd = 0;
            let mut args: String = "".to_string();
            let w = words.clone();
            if debug == "yes" {
                println!("{}", w.len().to_string().bright_yellow());
            }
            for word in words {
                if wrd == 0 && word == cmd {
                    continue;
                }
                else if wrd==w.len() {
                    args = args + word;
                }
                else {
                    args = args+word;
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
