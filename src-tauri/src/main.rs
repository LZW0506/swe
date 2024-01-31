// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod command;

use command::{test_source,source_db,down};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_source::test_data_source,source_db::query_table_info,down::down_word])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
