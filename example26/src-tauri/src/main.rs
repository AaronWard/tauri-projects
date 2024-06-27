// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// You can either relaunch from tauri rust api below
// or do it with `import { relaunch } from '@tauri-apps/api/process';`
// use tauri::{api::process::restart, Manager};


// #[tauri::command]
// fn restart(app: tauri::AppHandle) {
//     restart(&app.env());
// }


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())    
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
