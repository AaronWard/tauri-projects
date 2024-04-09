// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let devtools = devtools::init(); // initialize the plugin as early as possible

    tauri::Builder::default()
        .plugin(devtools) // then register it with Tauri
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// -- Used for disabling devtools when in a production build using cfg
// Rusts cfg macros can be used to conditionally disable or enable code. 
// In the following snippet we use them to disable the devtools code in non-debug builds, i.e. when you 
// build for production through cargo tauri build or cargo build --release.
// fn main() {
//     #[cfg(debug_assertions)]
//     let devtools = devtools::init(); // initialize the plugin as early as possible

//     let mut builder = tauri::Builder::default();

//     #[cfg(debug_assertions)]
//     builder = builder.plugin(devtools); // then register it with Tauri

//     builder.run(tauri::generate_context!("./tauri.conf.json"))
//         .expect("error while running tauri application");
// }