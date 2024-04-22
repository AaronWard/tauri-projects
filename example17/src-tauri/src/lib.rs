use tauri::{LogicalPosition, LogicalSize, Manager, WindowBuilder, AppHandle, WebviewUrl, Runtime };
use std::{ process::Command, sync::mpsc::channel, thread };

// use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::Window;

// async fn load_wasm_module(){
//     // fill in code here
// }  

pub fn run() {
    tauri::Builder
        ::default()
        .setup(move |app| {
            let app_handle = app.app_handle().clone();
            // load_wasm_file();
            let width = 1200.0;
            let height = 1000.0;
            let window = tauri::window::WindowBuilder
                ::new(app, "_main")
                .inner_size(width, height)
                .build()?;

            let _webview1 = window.add_child(
                tauri::webview::WebviewBuilder
                    ::new("app_content", WebviewUrl::App("index.html".into()))
                    .auto_resize(),
                LogicalPosition::new(width / 2.0, 0.0),
                LogicalSize::new(width / 2.0, height / 2.0)
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
