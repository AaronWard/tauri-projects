// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, AppHandle, Result};
use tauri::http::{ResponseBuilder};
use std::{fs::File, io::Read, path::PathBuf};

#[tauri::command]
fn load_plugin(name: String) -> Result<String> {
    let path = PathBuf::from(format!("./plugins/{}", name));
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_plugin])
        .register_uri_scheme_protocol("plugin", move |app_handle, request| {
            let path = request.uri().replace("plugin://", "");
            let path = PathBuf::from(format!("./src-tauri/plugins/{}", path));
            match File::open(&path) {
                Ok(mut file) => {
                    let mut contents = Vec::new();
                    if let Ok(_) = file.read_to_end(&mut contents) {
                        ResponseBuilder::new()
                            .mimetype("application/javascript")
                            .body(contents)
                    } else {
                        ResponseBuilder::new().status(404).body(Vec::new())
                    }
                },
                Err(_) => ResponseBuilder::new().status(404).body(Vec::new())
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
