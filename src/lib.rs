use colored::*;
#[allow(unused_imports)]
use std::{fs, process};

pub fn get_wakefiles(content: String) -> Vec<String> {
    let mut wakefiles: Vec<String> = Vec::new();
    let lines: Vec<&str> = content.split("\n").collect();
    let os = get_os();
    let mut sep = "";
    if os == "WINDOWS" {
        sep = "\\";
    } else if os == "LINUX" {
        sep = "/";
    }
    for line in lines {
        if line.contains(".Wakefile") {
            let fin = ".wake".to_string() + sep + line;
            wakefiles.push(fin.to_string());
        } else {
            println!("{}", "Warning: not a Wakefile!".yellow());
        }
    }
    return wakefiles;
}

pub fn get_instructions(content: String) -> String {
    let mut out: String = "".to_string();
    let lines: Vec<&str> = content.split("\n").collect();
    for line in lines {
        if !line.starts_with("#") {
            out = out + line + "\n";
        }
    }
    return out;
}

pub fn get_os() -> String {
    if fs::metadata("/etc/os-release").is_ok() {
        return "LINUX".to_string();
    } else if fs::metadata("C:\\Windows\\System32\\explorer.exe").is_ok() {
        return "WINDOWS".to_string();
    } else {
        return "unknown".to_string();
    }
}
