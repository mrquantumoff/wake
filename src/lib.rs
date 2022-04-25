#[allow(unused_imports)]
use std::{fs, process};

pub fn get_instructions(content: String) -> String {
    let mut out: String = "".to_string();
    let lines: Vec<&str> = content.split("\n").collect();
    for line in lines {
        if !line.starts_with("#") && !line.starts_with("!") {
            out = out + line;
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
