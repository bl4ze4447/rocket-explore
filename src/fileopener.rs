use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use egui::TextBuffer;

const DONT_SHOW_WINDOW: u32 = 0x08000000;

#[cfg(windows)]
pub fn open_file(file_path: &PathBuf) -> Result<(), String> {
    let start_process = "& {&'Start-Process' \"".to_owned() + file_path.to_string_lossy().as_str() + "\"}";
    let process = Command::new("PowerShell")
        .args(["-Command", start_process.as_str()]).creation_flags(DONT_SHOW_WINDOW).stderr(Stdio::null()).stdin(Stdio::null()).stdout(Stdio::null()).spawn();

    return match process {
        Ok(_) => {
            Ok(())
        },
        Err(_) => {
            Err("Could not open file. Path may not exist or PowerShell might not be installed".to_owned())
        },
    }
}

#[cfg(unix)]
pub fn open_file(file_path: &String) -> Result<(), String> {
    let mut process = Command::new("xdg-open").
        arg(file_path.as_str()).spawn();

    return match process {
        Ok(_) => {
            Ok(())
        },
        Err(err) => {
            Err(err.to_owned())
        }
    }
}
