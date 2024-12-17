use tauri::async_runtime::Mutex;

use shock_clock_utils::Block;
use tauri::State;

#[tauri::command]
pub async fn update_blocklist(
    state: State<'_, Mutex<Vec<shock_clock_utils::Block>>>,
    blocks: Vec<Block>,
) -> Result<(), ()> {
    // verrrry inefficient *purr*
    println!("yeah state");
    let mut lock = state.lock().await;
    lock.clear();
    blocks.into_iter().for_each(|block| lock.push(block));
    Ok(())
}
