use tauri::{LogicalPosition, LogicalSize, WebviewUrl };
use std::process::Command;
use std::path::PathBuf;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn serve_app_content() -> String {
//     // Start `example.app` in server mode
//     let _ = Command::new("./bundle/example.app")
//         .args(["--serve"])
//         .spawn()
//         .expect("Failed to start example.app");

//     // Return the local URL where the app serves its content
//     "http://localhost:3000".into()
// }

fn run_app() {

    match env::current_dir() {
        Ok(current_path) => {
            println!("Current working directory: {:?}", current_path);
        }
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
        }
    }
    // Locate the `.app` bundle
    let app_path = PathBuf::from("../src/example13.app");
    // Use the `open` command to run the `.app` bundle
    if app_path.exists() {
        Command::new("open")
            .arg(app_path)
            .spawn()
            .expect("Failed to start example13.app");
    } else {
        eprintln!("Failed to find example13.app at {:?}", app_path);
    }
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .setup(|app| {
            let width = 1200.0;
            let height = 1000.0;
            let window = tauri::window::WindowBuilder::new(app, "main")
            .inner_size(width, height)
            .build()?;

            // let content_url = serve_app_content();
            run_app();

            // let content_url = WebviewUrl::External(content_url);


            // Create a new webview for the app content
            // let webview_attrs = tauri_runtime::webview::WebviewAttributes(content_url);
            // let _webview1 =  window.add_child(
            //     tauri::webview::WebviewBuilder
            //         ::new("app_content", WebviewUrl::App(content_url.into())
            // )
            //     .auto_resize(),
            //     LogicalPosition::new(width / 2.0, 0.0),
            //     LogicalSize::new(width / 2.0, height / 2.0)
            // )?;
            let _webview2 = window.add_child(
                tauri::webview::WebviewBuilder
                    ::new("main2", WebviewUrl::App("index2.html".into()))
                    .auto_resize(),
                LogicalPosition::new(width / 2.0, 0.0),
                LogicalSize::new(width / 2.0, height / 2.0)
            )?;
            let _webview3 = window.add_child(
                tauri::webview::WebviewBuilder
                    ::new("main3", WebviewUrl::App("index3.html".into()))
                    .auto_resize(),
                LogicalPosition::new(0.0, height / 2.0),
                LogicalSize::new(width / 2.0, height / 2.0)
            )?;
            let _webview4 = window.add_child(
                tauri::webview::WebviewBuilder
                    ::new("main4", WebviewUrl::App("index4.html".into()))
                    .auto_resize(),
                LogicalPosition::new(width / 2.0, height / 2.0),
                LogicalSize::new(width / 2.0, height / 2.0)
            )?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
