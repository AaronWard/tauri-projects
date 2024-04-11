// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn create_app_menu() -> Menu {
    return Menu::new()
        .add_submenu(Submenu::new(
            "App",
            Menu::new().add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new(
            "Settings",
            Menu::new()
                .add_item(CustomMenuItem::new("new".to_string(), "New").accelerator("CmdOrCtrl+N"))
                .add_item(CustomMenuItem::new("open".to_string(), "Open").accelerator("CmdOrCtrl+O"))
                .add_native_item(MenuItem::Separator)
                .add_item(CustomMenuItem::new("save".to_string(), "Save").accelerator("CmdOrCtrl+S")),
        ));
}

fn main() {
    tauri::Builder::default()
        .menu(create_app_menu())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}