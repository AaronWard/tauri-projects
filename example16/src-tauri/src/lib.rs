#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowBuilder, WebviewUrl, LogicalPosition, LogicalSize, Runtime};
use std::{process::Command, thread};
use url::Url;

// Function to launch the external `.app`
fn launch_external_app() {
    thread::spawn(move || {
        Command::new("open")
            .arg("-a")
            .arg("/Users/award40/Desktop/personal/github/tauri-projects/example16/src/example13.app")
            .spawn()
            .expect("Failed to start .app file");
    });
}

// Function to execute AppleScript
fn run_applescript(script: &str) {
    Command::new("osascript")
        .args(&["-e", script])
        .spawn()
        .expect("Failed to execute AppleScript");
}

// Update the external app window based on the Tauri window state
fn update_external_app_position<R: Runtime>(app_handle: tauri::AppHandle<R>, label: &str, content_webview_label: &str) {
    if let Some(main_window) = app_handle.get_window(label) {
        if let Some(content_window) = main_window.get_window(content_webview_label) {
            let main_position = main_window.outer_position().unwrap();
            let content_position = content_window.outer_position().unwrap();
            let content_size = content_window.outer_size().unwrap();

            let script = format!(
                r#"
                tell application "System Events" to tell process "example13"
                    set position of the first window to {{{}, {}}}
                    set size of the first window to {{{}, {}}}
                end tell
                "#,
                main_position.x + content_position.x, main_position.y + content_position.y,
                content_size.width, content_size.height
            );
            run_applescript(&script);
        }
    }
}

// Main function that sets up the Tauri app
pub fn run() {
    tauri::Builder::default()
        .setup(move |app| {
            let width = 1200.0;
            let height = 1000.0;
            let app_handle = app.app_handle().clone();
            launch_external_app(); // Launch the external app
            
            // Create the main Tauri window
            let window = WindowBuilder::new(app, "main")
                .inner_size(width, height)
                .build()
                .expect("Failed to build main window");

            // Create the webview that the external app will overlay
            let content_webview_label = "app_content";
            let content_url = Url::parse("http://localhost:4000").expect("Failed to parse URL");
            let _webview1 = window.add_child(
                tauri::webview::WebviewBuilder::new(content_webview_label, WebviewUrl::External(content_url))
                    .auto_resize(),
                LogicalPosition::new(width / 2.0, 0.0),
                LogicalSize::new(width / 2.0, height / 2.0)
            ).expect("Failed to add child webview");

            // Listen for window resize and move events
            app_handle.listen("tauri://resize", move |_| {
                update_external_app_position(app_handle.clone(), "main", content_webview_label);
            });
            app_handle.listen("tauri://move", move |_| {
                update_external_app_position(app_handle.clone(), "main", content_webview_label);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![/* your command handlers here */])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
