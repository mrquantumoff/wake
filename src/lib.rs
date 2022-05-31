use colored::*;
use std::path::PathBuf;
#[allow(unused_imports)]
use std::{fs, path, process};

pub fn run_unified(command: String, _debug: String) {
    let os = get_os();
    if os == "WINDOWS" {
        println!("{}", "Running on Windows".bright_green());
        windows::run_pwsh(command, &_debug);
    } else if os == "LINUX" {
        println!("{}", "Running on GNU/Linux".bright_green());
        linux::run_bash(command, _debug);
    } else {
        println!("{}", "Unknown OS, aborting!".bright_red());
        process::exit(1);
    }
}

pub fn get_wakefiles(content: String) -> Vec<PathBuf> {
    let mut wakefiles: Vec<PathBuf> = Vec::new();
    let lines: Vec<&str> = content.split("\n").collect();
    for line in lines {
        if line.contains(".Wakefile") {
            //let fin = ".wake".to_string() + sep + line;
            let fin = path::Path::new(".wake").join(line.trim());
            wakefiles.push(fin);
        } else {
            println!("{}", "Warning: not a Wakefile!".yellow());
        }
    }
    return wakefiles;
}

pub fn get_os() -> String {
    let os = std::env::consts::OS;
    if os == "windows" {
        return "WINDOWS".to_string();
    } else if os == "linux" {
        return "LINUX".to_string();
    } else {
        return "other".to_string();
    }
}
pub mod windows {
    use colored::*;
    pub fn run_pwsh(command: String, _debug: &String) {
        let lines: Vec<&str> = command.split("\n").collect();
        for line in lines {
            if line.contains(" && ") || line.contains(" | ") || line.contains(" >> ") {
                println!(
                    "{}",
                    "Some powershell syntax is not supported skipping the line!".bright_red()
                )
            } else if line == "" {
            } else {
                println!("{} {}", "Executing".bright_yellow(), line.bright_blue());
                let out = std::process::Command::new("powershell")
                    .arg("-Command")
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
}
pub mod linux {
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
}
