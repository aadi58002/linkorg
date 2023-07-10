// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod org;

use std::path::PathBuf;

use crate::{config::config::parse_config, org::file::find_org_files};
use config::config::Config;
use dirs;
use org::parse::{read_org_file, FileData};

#[tauri::command]
fn get_config() -> Config {
    let config_path = dirs::config_dir()
        .unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join("config.toml");
    parse_config(config_path)
}

#[tauri::command]
fn get_files_list(notes_dir: PathBuf) -> Vec<PathBuf> {
    find_org_files(notes_dir)
}

#[tauri::command]
fn get_file_data(file: PathBuf) -> FileData {
    read_org_file(file)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_file_data,
            get_files_list,
            get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn main() {
//     let config_path = dirs::config_dir()
//         .unwrap()
//         .join(env!("CARGO_PKG_NAME"))
//         .join("config.toml");
//     let config = parse_config(config_path);
//     let files = find_org_files(config.notes_dir);
//     let _ = read_org_file(files[0].clone());
//     // println!("{:#?}", read_org_file(files[0].clone()));
// }
