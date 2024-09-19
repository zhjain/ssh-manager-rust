// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod session;
mod sled;
mod ssh;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            sled::handle_db_operation,
            ssh::handle_ssh_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
