// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use utils::set_window_shadow;

mod session;
mod sled;
mod ssh;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            sled::handle_db_operation,
            ssh::handle_ssh_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
