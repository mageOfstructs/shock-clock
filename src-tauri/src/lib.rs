mod watcher_state;
use tauri::async_runtime::Mutex;
use tauri::Manager;

mod ble;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_blec::init())
        .manage(Mutex::new(Vec::<shock_clock_utils::Block>::new()))
        // .manage(Mutex::new(Option::<String>::None))
        .invoke_handler(tauri::generate_handler![
            watcher_state::update_blocklist,
            greet,
            ble::shock,
            ble::is_connected
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
