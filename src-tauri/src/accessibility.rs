use std::sync::Arc;
use std::thread;

use shock_clock_utils::{Block, BlockType, ShockStrength};
use tauri::async_runtime::{self, Mutex};
use tauri::{AppHandle, State};
use tauri_plugin_accessibility::EventPayload;
use tauri_plugin_accessibility::{AccessibilityEvent, AccessibilityExt};
use tokio::sync::MutexGuard;

#[tauri::command]
pub fn init_accessibility(
    app: AppHandle,
    state: State<'_, Arc<Mutex<Vec<shock_clock_utils::Block>>>>,
) -> Result<(), ()> {
    let state = Arc::clone(&state);
    thread::spawn(move || {
        async_runtime::block_on(async {
            println!("init accessibility service");

            loop {
                let event = app.accessibility().get_event(EventPayload).unwrap();
                if event.text != "" {
                    check_for_block(event, &state.lock().await).await;
                }
            }
        });
    });
    Ok(())
}

async fn check_for_block(
    accessibility_event: AccessibilityEvent,
    blocks: &MutexGuard<'_, Vec<Block>>,
) {
    println!("{}", blocks.len());
    for block in blocks.iter() {
        let blocked = match &block.block_type {
            BlockType::Keyword => process_keyword(&accessibility_event, block.name.clone()).await,
            BlockType::Website(data) => {
                process_keyword(&accessibility_event, data.url.clone()).await
            }
            BlockType::App(data) => {
                process_app(&accessibility_event, data.package_name.clone()).await
            }
        };
        if blocked {
            println!(
                "Blocked!!!: {:?} {:?}",
                block.block_type, block.shock_strength
            );
            break;
        }
    }
}

async fn process_keyword(accessibility_event: &AccessibilityEvent, keyword: String) -> bool {
    println!(
        "Keyword: {}, compared with {}",
        keyword, accessibility_event.text
    );
    accessibility_event.text.contains(&keyword)
}

async fn process_app(accessibility_event: &AccessibilityEvent, package: String) -> bool {
    // if accessibility_event.package != "com.shock_clock.app" {}
    true
}
