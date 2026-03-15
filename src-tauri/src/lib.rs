pub mod state;
pub mod commands;

use std::sync::Mutex;
use state::TerminalState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(TerminalState::new()))
        .invoke_handler(tauri::generate_handler![commands::execute_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
