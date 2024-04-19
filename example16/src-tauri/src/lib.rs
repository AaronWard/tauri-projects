#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ Manager, WindowBuilder, Runtime, LogicalPosition, LogicalSize, Size };
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
            .expect("Failed to open the external .app");
    });
}

// Function to execute AppleScript
fn run_applescript(script: &str) {
    Command::new("osascript")
        .args(&["-e", script])
        .spawn()
        .expect("Failed to execute AppleScript command");
}

// Update the external app window based on the Tauri window state
fn update_external_app_position<R: Runtime>(app_handle: &tauri::AppHandle<R>, target_width: f64, target_height: f64) {
    let main_window = app_handle.get_window("main").expect("Failed to get main window");

    // Retrieve the current position of the main window
    let main_position = main_window.outer_position().expect("Failed to get window position");

    // AppleScript to move and resize the external .app window
    let script = format!(
        r#"
        tell application "System Events" to tell process "example13"
            set position of the first window to {{{}, {}}}
            set size of the first window to {{{}, {}}}
        end tell
        "#,
        main_position.x as i32, // Position x
        main_position.y as i32, // Position y
        target_width as i32, // Width of the .app window
        target_height as i32 // Height of the .app window
    );

    run_applescript(&script);
}

pub fn run() {
    tauri::Builder
        ::default()
        .setup(move |app| {
            let app_handle = app.app_handle().clone();
            launch_external_app(); // Launch the external app
            let target_width: f64 = 600.0; // Half width of the Tauri window
            let target_height: f64 = 500.0; // Half height of the Tauri window
            
            // Determine the size of the Tauri window where the external app will be placed
            // These values may need adjustment to align with the actual design requirements

            let width = 1200.0;
            let height = 1000.0;

            let _window = tauri::window::WindowBuilder
                ::new(app, "main")
                .inner_size(width, height)
                .build()
                .expect("Failed to build window");

            // Set the size of the quadrant for the .app to overlay
            update_external_app_position(&app_handle, target_width, target_height); // Set the initial position of the .app

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
                    update_external_app_position(&app_handle, target_width,target_height);
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
}
