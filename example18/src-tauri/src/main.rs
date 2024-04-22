// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Builder, Manager, WindowUrl, AppHandle};
use tauri_plugin_localhost::{Builder as LocalhostBuilder};
use tauri_invoke_http::InvokeHttp;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;


#[tauri::command]
async fn greet_command(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(LocalhostBuilder::default().build())
        .setup(move |app| {
            let handle = app.handle();
            setup_invoke_http(app);
            open_window(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_invoke_http(app: &AppHandle) {
    let invoke_http = InvokeHttp::new(["http://localhost:1420"]);
    invoke_http.start(app.clone());
}

fn open_window(app_handle: AppHandle) {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    let url = format!("http://localhost:{}", port);
    WindowUrl::External(url.parse().unwrap());

    tauri::WindowBuilder::new(
        &app_handle, 
        "main", 
        WindowUrl::External(url.parse().unwrap())
    )
    .title("Localhost Example")
    .build()
    .expect("error while creating window");
}
