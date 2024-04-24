// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Import necessary Tauri and Actix-web components
use tauri::{Manager, Window, AppHandle, Builder};
use actix_web::{web, App as ActixApp, HttpServer, Responder, HttpResponse};
use actix_files as actix_files; // For serving static files

// Correcting the usage of Tauri's Window and its builder
use tauri::api::window::WindowBuilder;  // Correct way to import WindowBuilder

#[tauri::command]
async fn greet_command(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            // Get the app handle and start the HTTP server
            let handle = app.handle();
            tauri::async_runtime::spawn(run_server());
            // Open a window using the handle
            open_window(handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Function to open a window
fn open_window(app_handle: AppHandle) {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    let url = format!("http://localhost:{}", port);

    WindowBuilder::new(&app_handle, "main", tauri::WindowUrl::External(url.parse().unwrap()))
        .title("Localhost Example")
        .build()
        .expect("error while creating window");
}

// Function to run the HTTP server
async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        ActixApp::new()
            .route("/", web::get().to(root))
            .service(actix_files::Files::new("/", "./dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?  // Binding to localhost on a specific port
    .run()
    .await
}

// Simple HTTP handler for root requests
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello from the server!")
}
