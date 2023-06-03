// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

mod config;
mod org;

use crate::{config::config::parse_config, org::file::find_org_files};
use dirs;
use org::parse::parse_org_file;

fn main() {
    let config_path = dirs::config_dir()
        .unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join("config.toml");
    let config = parse_config(config_path);
    let files = find_org_files(config.notes_dir);
    for file in files {
        let _ = parse_org_file(file);
    }
}
