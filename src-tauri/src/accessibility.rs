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
        println!("init accessibility service");
        let mut lock = state.lock().await;

        loop {
            let event = app.accessibility().get_event(EventPayload).unwrap();
            if event.text != "" {
                println!(
                    "{}\n{}\n{}\n\n",
                    event.package, event.event_type, event.text
                );
                println!("thread spawned");
                println!("{}", lock.len());

                check_for_block(event, &lock).await;
            }
        }
    });
    Ok(())
}

async fn check_for_block(
    accessibility_event: AccessibilityEvent,
    blocks: &MutexGuard<'_, Vec<Block>>,
) {
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
            println!("Block: {:?} {:?}", block.block_type, block.shock_strength);
            break;
        }
    }
}

async fn process_keyword(accessibility_event: &AccessibilityEvent, keyword: String) -> bool {
    // accessibility_event.text.contains(&keyword)
    true
}

async fn process_app(accessibility_event: &AccessibilityEvent, package: String) -> bool {
    if accessibility_event.package != "com.shock_clock.app" {}
    true
}
