#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value as JsonValue;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::{Store, StoreBuilder};

type WryStore = Store<tauri::Wry>;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// async fn save_data(key: String, value: JsonValue, app: AppHandle) -> Result<(), String> {
//     let store = app.state::<WryStore>().expect("Store state not found");
//     store.insert(key, value).map_err(|e| e.to_string()) // Correctly pass key as owned String
// }

// #[tauri::command]
// async fn retrieve_data(key: String, app: AppHandle) -> Result<JsonValue, String> {
//     let store = app.state::<WryStore>().expect("Store state not found");
//     store.get(&key).ok_or_else(|| "Key not found".to_string()) // Correct handling of Option
// }
