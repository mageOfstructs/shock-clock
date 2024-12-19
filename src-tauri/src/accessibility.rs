use shock_clock_utils::{Block, BlockType, ShockStrength};
use tauri::async_runtime::{self, Mutex};
use tauri::{AppHandle, State};
use tauri_plugin_accessibility::EventPayload;
use tauri_plugin_accessibility::{AccessibilityEvent, AccessibilityExt};
use tokio::sync::MutexGuard;

#[tauri::command]
pub async fn init_accessibility(
    app: AppHandle,
    state: State<'_, Mutex<Vec<shock_clock_utils::Block>>>,
) -> Result<(), ()> {
    async_runtime::block_on(async {
        let mut lock = state.lock().await;

        loop {
            let event = app.accessibility().get_event(EventPayload).unwrap();
            if event.text != "" {
                println!(
                    "{}\n{}\n{}\n\n",
                    event.package, event.event_type, event.text
                );
                async_runtime::block_on(async {
                    println!("thread spawned");
                    println!("{}", lock.len());

                    check_for_block(event, lock);
                });
            }
        }
    });
    Ok(())
}

async fn check_for_block(
    accessibility_event: AccessibilityEvent,
    blocks: MutexGuard<'_, Vec<Block>>,
) {
    blocks.iter().for_each(|block| {
        match block.block_type {
            BlockType::Keyword => 
        }
    });
}
