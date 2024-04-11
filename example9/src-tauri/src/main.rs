#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::command;
use std::process::Command; // Use std::process::Command for external command execution

#[command]
async fn greet(name: String) -> Result<String, String> { // Use String as the error type for simplicity
    let output = Command::new("./binaries/greet-aarch64-apple-darwin") // Adjust the path as necessary
        .arg(name)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned()) // Convert output to String directly
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned()) // Return errors as Err
    }
}

fn main() {
    let devtools = devtools::init(); // initialize the plugin as early as possible

    tauri::Builder::default()
        .plugin(devtools) 
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
