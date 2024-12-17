use std::sync::Mutex;

use shock_clock_utils::Block;
use tauri::State;

#[tauri::command]
pub fn update_blocklist(
    state: State<'_, Mutex<Vec<shock_clock_utils::Block>>>,
    blocks: Vec<Block>,
) {
    // verrrry inefficient *purr*
    println!("yeah state");
    let mut lock = state.lock().unwrap();
    lock.clear();
    blocks.into_iter().for_each(|block| lock.push(block));
}
