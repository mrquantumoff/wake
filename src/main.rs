#[allow(unused_imports)]
use clap::*;
use colored::*;
use std::{env, fs, process};
mod lib;
mod linux;
mod windows;
fn main() {
    let matches = command!()
        .arg(
            arg!(
                -s --source <FILE> "Instruction source"
            )
            .required(false)
            .default_value("WakeFileList")
            .allow_invalid_utf8(false),
        )
        .arg(
            arg!(
                --debug <BOOL> "Show debug info"
            )
            .required(false)
            .allow_invalid_utf8(false)
            .possible_values(&["yes", "no"])
            .default_value("no"),
        )
        .arg(
            arg!(
                -n --new <PROJECTNAME> "Create a new project"
            )
            .required(false)
            .allow_invalid_utf8(false)
            .default_value("!WAKENOPROJECT!"),
        )
        .get_matches();
    let source = matches.value_of("source").unwrap();
    let debug = matches.value_of("debug").unwrap();
    let new = matches.value_of("new").unwrap();
    if new != "!WAKENOPROJECT!" {
        // Create a new project
        // Create a new path
        let fpath = fs::create_dir(new);
        if !fpath.is_ok() {
            println!("{}", "Error: could not create the project!".bright_red());
            process::exit(1);
        }
        // Create a new file
        let os = lib::get_os();
        let mut sep = "";
        if os == "WINDOWS" {
            sep = "\\";
        } else if os == "LINUX" {
            sep = "/";
        }
        fs::File::create(new.to_string() + sep + "WakeFileList").unwrap();
        let wr = fs::write(new.to_string() + sep + "WakeFileList", "main.Wakefile");
        if wr.is_err() {
            println!("{}", "Error: could not create the project!".bright_red());
            process::exit(1);
        }
        // Create .wake folder
        let tpath = fs::create_dir(new.to_string() + sep + ".wake");
        if !tpath.is_ok() {
            println!("{}", "Failed to create .wake folder".bright_red());
            std::process::exit(1);
        }
        fs::File::create(new.to_string() + sep + ".wake" + sep + "main.Wakefile").unwrap();
        println!("{}", "Project created!".bright_green());
    } else {
        if fs::metadata(source).is_ok() {
            if source.ends_with(".Wakefile") {
                let raw = fs::read_to_string(source).unwrap();
                let r = raw.clone();
                let instructions = lib::get_instructions(raw);
                if debug == "yes" {
                    println!("{}", instructions.bright_yellow());
                    println!("{}", r.bright_red());
                }
                println!("{}", instructions);
                let os = lib::get_os();
                if os == "WINDOWS" {
                    println!("{}", "Running on Windows".bright_blue());
                } else if os == "LINUX" {
                    println!("{}", "Running on GNU/Linux".bright_green());
                    linux::run_bash(instructions, debug.to_string());
                } else {
                    println!("{}", "Unknown OS, aborting!!!".bright_red());
                    process::exit(1);
                }
            } else if source == "WakeFileList" {
                let raw = fs::read_to_string(source).unwrap();
                let wakefiles = lib::get_wakefiles(raw);
                for wakefile in wakefiles {
                    let w = fs::read_to_string(wakefile).unwrap();
                    let rr = w.clone();
                    let instructions = lib::get_instructions(w);
                    if debug == "yes" {
                        println!("{}", instructions.bright_yellow());
                        println!("{}", rr.bright_red());
                    }
                    println!("{}", instructions);
                    let os = lib::get_os();
                    if os == "WINDOWS" {
                        println!("{}", "Running on Windows".bright_blue());
                    } else if os == "LINUX" {
                        println!("{}", "Running on GNU/Linux".bright_green());
                        linux::run_bash(instructions, debug.to_string());
                    } else {
                        println!("{}", "Unknown OS, aborting!!!".bright_red());
                        process::exit(1);
                    }
                }
            } else {
                println!("{}", "Not a Wakefile!".bright_red());
                std::process::exit(1);
            }
        } else {
            println!("{}", "File not found!".bright_red());
            std::process::exit(1);
        }
    }
}
