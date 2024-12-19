mod watcher_state;
use std::thread;
use std::time::Duration;
use tauri::async_runtime;
use tauri::async_runtime::Mutex;
use tauri::{Builder, Manager};
use tauri_plugin_accessibility::AccessibilityExt;
use tauri_plugin_accessibility::EventPayload;

mod ble;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Vec::<shock_clock_utils::Block>::new()))
        .manage(Mutex::new(Option::<String>::None))
        .setup(|app| {
            let accessibility = app.accessibility();
            let state = app.state::<Mutex<Vec<shock_clock_utils::Block>>>();

            loop {
                let event = accessibility.get_event(EventPayload).unwrap();
                if event.text != "" {
                    println!(
                        "{}\n{}\n{}\n\n",
                        event.package, event.event_type, event.text
                    );
                    async_runtime::block_on(async {
                        let blocks = state.lock().await;
                        println!("{}", blocks.len());
                    });
                }
            }

            // app.listen_any("accessibilityEvent", |event| {
            //     if let Ok(payload) = serde_json::from_str::<AccessibilityEvent>(&event.payload()) {
            //         println!("New event {}", payload.message);
            //     }
            // });
            // Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_accessibility::init())
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
