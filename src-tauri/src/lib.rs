mod watcher_state;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;
use tauri::Listener;

mod ble;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize, Serialize)]
struct AccessibilityEvent {
    message: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            println!("starting to listen lol");
            app.listen_any("accessibilityEvent", |event| {
                if let Ok(payload) = serde_json::from_str::<AccessibilityEvent>(&event.payload()) {
                    println!("New event {}", payload.message);
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_accessibility::init())
        .manage(Mutex::new(Vec::<shock_clock_utils::Block>::new()))
        .manage(Mutex::new(Option::<String>::None))
        .invoke_handler(tauri::generate_handler![
            watcher_state::update_blocklist,
            greet,
            ble::shock,
            ble::is_connected,
            ble::init_scanloop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
