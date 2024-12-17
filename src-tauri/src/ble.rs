use shock_clock_utils::ble::IsConnected;
use tauri::async_runtime::Mutex;
use tauri::{async_runtime, State};
use tokio::sync::mpsc;
use uuid::{uuid, Uuid};

const DEV_NAME: &str = "Shock Clock";
const SHOCK_FLAG: Uuid = uuid!("155dc6c3-99c5-4f87-aa9d-329fcfaf893b");
const LOOP_CHARA: Uuid = uuid!("873346bd-a08b-4769-b006-4375190f6bc7");
const COOLDOWN_CHARA: Uuid = uuid!("1d0edd21-dfce-4906-8a47-7cf83aef1292");

async fn scan() -> Option<String> {
    let handler = tauri_plugin_blec::get_handler().unwrap();
    let (tx, mut rx) = mpsc::channel(1);
    handler.lock().await.discover(Some(tx), 1000).await.unwrap();
    while let Some(devices) = rx.recv().await {
        for dev in devices.iter() {
            println!("Device: {dev:?}");
            if dev.name == DEV_NAME && !dev.is_connected {
                println!("Found clock: {}", dev.address);
                return Some(dev.address.clone());
            }
        }
    }
    None
}

async fn connect(address: String) {
    let handler = tauri_plugin_blec::get_handler().unwrap();
    match handler
        .lock()
        .await
        .connect(address, Some(|| println!("disconnected!")))
        .await
    {
        Ok(_) => println!("Successfully connected!"),
        Err(err) => println!("Error during connection: {err}"),
    }
}

// FIXME: this doesn't work anymore if it returns anything :c
#[tauri::command]
pub fn shock(state: State<'_, Mutex<Option<String>>>, duration: u16) {
    let mut res = Err(());
    async_runtime::block_on(async {
        if let Some(ref addr) = *state.lock().await {
            connect(addr.to_string()).await;
        } else {
            eprintln!("Don't have an address yet!");
            return;
        }
        let mut handler = tauri_plugin_blec::get_handler().unwrap().lock().await;
        if let Ok(_) = handler.connected_device().await {
            // no idea if little endian works here
            let data = [(duration & 255) as u8, (duration >> 8) as u8];
            if let Err(err) = handler.send_data(SHOCK_FLAG, &data).await {
                eprintln!("While sending data: {err}");
            }
            res = Ok(());
        }
    });
}

#[tauri::command]
pub async fn is_connected(state: State<'_, Mutex<Option<String>>>) -> Result<IsConnected, ()> {
    println!("Seeing if device is connected...");
    let mut state = state.lock().await;
    if let None = *state {
        *state = scan().await;
    }
    Ok(IsConnected(state.is_some()))
}
