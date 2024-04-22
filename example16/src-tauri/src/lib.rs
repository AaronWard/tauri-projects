use tauri::{LogicalPosition, LogicalSize, Manager, WindowBuilder, AppHandle, WebviewUrl, Runtime };
use std::{ process::Command, sync::mpsc::channel, thread };

// Function to launch the external `.app`
fn launch_external_app() {
    thread::spawn(move || {
        Command::new("open")
            .arg("-a")
            .arg(
                "/Users/award40/Desktop/personal/github/tauri-projects/example16/src/example13.app"
            )
            .spawn()
            .expect("Failed to start .app file");
    });
}

// Function to execute AppleScript
fn run_applescript(script: &str) {
    Command::new("osascript").args(&["-e", script]).spawn().expect("Failed to execute AppleScript");
}

// Update the external app window based on the Tauri window state
fn update_external_app_position<R: Runtime>(app_handle: &tauri::AppHandle<R>) {
    if let Some(window) = app_handle.get_window("example13") {
        let main_position = window.outer_position().unwrap();
        let main_size = window.outer_size().unwrap();

        let script = format!(
            r#"
            tell application "System Events" to tell process "example13 Graphics and Media"
                set position of the first window to {{{}, {}}}
                set size of the first window to {{{}, {}}}
            end tell
            "#,
            main_position.x,
            main_position.y,
            main_size.width / 2,
            main_size.height
        );
        run_applescript(&script);
    }
}

// Make the `run` function public to be accessed from `main.rs`
pub fn run() {
    tauri::Builder
        ::default()
        .setup(move |app| {
            let app_handle = app.app_handle().clone();
            launch_external_app();
            let width = 1200.0;
            let height = 1000.0;
            let window = tauri::window::WindowBuilder
                ::new(app, "main")
                .inner_size(width, height)
                .build()?;

            let _webview1 = window.add_child(
                tauri::webview::WebviewBuilder
                    ::new("app_content", WebviewUrl::App("index.html".into()))
                    .auto_resize(),
                LogicalPosition::new(width / 2.0, 0.0),
                LogicalSize::new(width / 2.0, height / 2.0)
            )?;


            // Create an event channel to communicate window move and resize events
            let (tx, rx) = channel::<()>();

            {
                let tx_clone = tx.clone(); // Clone the sender before moving it into the closure
                app_handle.listen("tauri://move", move |_| {
                    let _ = tx_clone.send(());
                });
            }

            {
                let tx_clone = tx; // Reuse the last clone of the sender
                app_handle.listen("tauri://resize", move |_| {
                    let _ = tx_clone.send(());
                });
            }

            // Handle window events in a new thread to update the position of the external .app
            thread::spawn(move || {
                for _ in rx {
                    update_external_app_position(&app_handle);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
