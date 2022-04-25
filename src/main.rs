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
            .required(true)
            .allow_invalid_utf8(false),
        )
        .get_matches();
    let source = matches.value_of("source").unwrap();
    if source.ends_with(".Wakefile") {
        let raw = fs::read_to_string(source).unwrap();
        let instructions = lib::get_instructions(raw);
        let os = lib::get_os();
        if os == "WINDOWS" {
            println!("{}", "Running on Windows".bright_blue());
        } else if os == "LINUX" {
            println!("{}", "Running on GNU/Linux".bright_green());
            linux::run_bash(instructions);
        } else {
            println!("{}", "Unknown OS, aborting!!!".bright_red());
            process::exit(-1);
        }
    }
    else {
        println!("{}","Not a Wakefile!".bright_red());
        std::process::exit(-1);
    }
}
