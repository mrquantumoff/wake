#[allow(unused_imports)]
use clap::*;
use colored::*;
use std::{env, fs, fs::File, io::Read, path, process};
mod lib;

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
                --debug <yes_or_no> "Show debug info"
            )
            .required(false)
            .allow_invalid_utf8(false)
            .possible_values(&["yes", "no"])
            .default_value("no"),
        )
        .subcommand(
            Command::new("new")
                .about("Create a new WAKE project")
                .arg(
                    arg!(
                        -n --name <NAME> "Project name"
                    )
                    .required(true)
                    .allow_invalid_utf8(false),
                )
                .arg(
                    arg!(
                        -l --language <LANGUAGE> "Language"
                    )
                    .required(true)
                    .allow_invalid_utf8(false),
                ),
        )
        .get_matches();
    let source = matches.value_of("source").unwrap();
    let debug = matches.value_of("debug").unwrap();
    let mut is_new = false;
    if let Some(subc) = matches.subcommand_matches("new") {
        is_new = true;
        let new = subc.value_of("name").unwrap();
        let lang = subc.value_of("language").unwrap();
        
            // Create a new project
            // Create a new path
            let fpath = fs::create_dir(new);
            if !fpath.is_ok() {
                println!("{}", "Error: could not create the project!".bright_red());
                process::exit(1);
            }
            // Create a new file
            let os = lib::get_os();
            let fpath = path::Path::new(new).join("WakeFileList");
            fs::File::create(&fpath).unwrap();
            let wr = fs::write(&fpath, "main.Wakefile");
            if wr.is_err() {
                println!("{}", "Error: could not create the project!".bright_red());
                process::exit(1);
            }
            // Create .wake folder
            let ffpath = path::Path::new(new).join(".wake");
            if debug == "yes" {
                println!("{}", ffpath.to_str().unwrap().bright_yellow());
            }
            let tpath = fs::create_dir(&ffpath);
            if !tpath.is_ok() {
                println!("{}", "Failed to create .wake folder".bright_red());
                std::process::exit(1);
            }
            let mpath = ffpath.join("main.Wakefile");
            fs::File::create(mpath).unwrap();
            match lang {
                "rust" => {
                    // run cargo init
                    let cmd = process::Command::new("cargo")
                        .arg("init")
                        .arg(new)
                        .output()
                        .expect("Failed to create a rust project");
                    println!("{}", String::from_utf8_lossy(&cmd.stdout));
                }
                "other" => {
                    let p = path::Path::new(new).join("src");
                    fs::create_dir(p).unwrap();
                }
                "python" => {
                    let p = path::Path::new(new).join("src");
                    fs::create_dir(&p).unwrap();
                    fs::File::create(p.join("main.py")).unwrap();
                }
                "dotnet" => {
                    env::set_current_dir(new).unwrap();
                    let cmd = process::Command::new("dotnet")
                        .arg("new")
                        .arg("console")
                        .output()
                        .expect("Failed to create a dotnet project");
                    println!("{}", String::from_utf8_lossy(&cmd.stdout));
                }
                &_ => {
                    println!("{}", "Language not supported yet".bright_red());
                    process::exit(1);
                }
            }
            if os == "LINUX" {
                if fs::metadata("/bin/git").is_ok() {
                    println!("{}", "Git repo initialized!".bright_green());
                    #[allow(unused_variables)]
                    let out = process::Command::new("/bin/git")
                        .arg("init")
                        .arg(new)
                        .output()
                        .expect("Failed to initialize git repository");
                }
            }
            println!("{}", "Project created!".bright_green());
        }
    if fs::metadata(source).is_ok() && !is_new {
        if source.ends_with(".Wakefile") {
            let raw = fs::read_to_string(source).unwrap();
            let r = raw.clone();
            if debug == "yes" {
                println!("{}", raw.bright_yellow());
                println!("{}", r.bright_red());
            }
            println!("{}", raw);
            let os = lib::get_os();
            if os == "WINDOWS" {
                println!("{}", "Running on Windows".bright_green());
                lib::windows::run_pwsh(raw, &debug.to_string());
            } else if os == "LINUX" {
                println!("{}", "Running on GNU/Linux".bright_green());
                lib::linux::run_bash(raw, debug.to_string());
            } else {
                println!("{}", "Unknown OS, aborting!".bright_red());
                process::exit(1);
            }
        } else if source == "WakeFileList" {
            if debug == "yes" {
                println!("{}", "WakeFileList is the default file".bright_yellow());
            }
            let raw = fs::read_to_string(source).unwrap();
            let wakefiles = lib::get_wakefiles(raw);
            if debug == "yes" {
                println!("{}", "Got wakefiles".bright_yellow());
            }
            for wakefile in wakefiles {
                if debug == "yes" {
                    println!("\"{}\"", &wakefile.to_str().unwrap());
                }
                let mut raw = File::open(&wakefile).expect("Failed to open file");
                let mut _w = String::new();
                _w = _w.trim().to_string();
                raw.read_to_string(&mut _w).expect("Failed to read file");
                lib::run_unified(_w, debug.to_string());
            }
        } else {
            println!("{}", "Not a Wakefile!".bright_red());
            std::process::exit(1);
        }
    } 
    else if is_new {
        std::process::exit(0);
    } else {
        println!("{}", "File not found!".bright_red());
        std::process::exit(1);
    }
}
