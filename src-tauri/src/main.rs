// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod file;

use std::path::PathBuf;

use crate::{
    config::config::{parse_config, Config},
    file::{file::*, parse::*},
};
use dirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref COMMON_FOLDER_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub static ref CONFIG_PATH_GLOBAL: PathBuf = dirs::config_dir()
        .expect("Unable to get Config Path")
        .join::<&'static str>(COMMON_FOLDER_NAME.as_ref())
        .join("config.toml");
}

#[tauri::command]
fn get_config() -> Config {
    parse_config(&CONFIG_PATH_GLOBAL)
}

#[tauri::command]
fn get_files_list(notes_dir: PathBuf) -> Vec<PathBuf> {
    find_data_files(notes_dir)
}

#[tauri::command]
fn get_file_data(file: PathBuf) -> FileData {
    read_data_file(file)
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
//     let files = find_data_files(config.notes_dir);
//     let _ = read_data_file(files[0].clone());
//     // println!("{:#?}", read_data_file(files[0].clone()));
// }
