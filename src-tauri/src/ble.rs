use std::thread;
use std::time::Duration;

use shock_clock_utils::ble::IsConnected;
use tauri::async_runtime::Mutex;
use tauri::{async_runtime, AppHandle, Emitter, Manager, State};
use tokio::sync::mpsc;
use uuid::{uuid, Uuid};

const DEV_NAME: &str = "Shock Clock";
const SHOCK_FLAG: Uuid = uuid!("155dc6c3-99c5-4f87-aa9d-329fcfaf893b");
const LOOP_CHARA: Uuid = uuid!("873346bd-a08b-4769-b006-4375190f6bc7");
const COOLDOWN_CHARA: Uuid = uuid!("1d0edd21-dfce-4906-8a47-7cf83aef1292");

async fn scan() -> Option<String> {
    let handler = tauri_plugin_blec::get_handler().unwrap();
    let (tx, mut rx) = mpsc::channel(1);
    if let Err(err) = handler.lock().await.discover(Some(tx), 1000).await {
        eprintln!("While scanning: {err}");
        return None;
    }
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
        eprintln!("attempting shock...");
        let mut handler = tauri_plugin_blec::get_handler().unwrap().lock().await;
        eprintln!("got lock...");

        if let Ok(_) = handler.connected_device().await {
            let data = [(duration & 255) as u8, (duration >> 8) as u8];
            match handler.send_data(SHOCK_FLAG, &data).await {
                Ok(_) => println!("data sent"),
                Err(err) => eprintln!("While sending data: {err}"),
            }
            res = Ok(());
        } else {
            eprintln!("Device disconnected prematurely!"); // this is a straight up lie
            *state.lock().await = None;
        }
    });
}

#[tauri::command]
pub fn shock_loop(
    state: State<'_, Mutex<Option<String>>>,
    iterations: u8,
    cooldown: u8,
    duration: u16,
) {
    let mut res = Err(());
    async_runtime::block_on(async {
        if let Some(ref addr) = *state.lock().await {
            connect(addr.to_string()).await;
        } else {
            eprintln!("Don't have an address yet!");
            return;
        }
        eprintln!("attempting shock...");
        let mut handler = tauri_plugin_blec::get_handler().unwrap().lock().await;
        eprintln!("got lock...");

        if let Ok(_) = handler.connected_device().await {
            match handler.send_data(LOOP_CHARA, &[iterations]).await {
                Ok(_) => println!("loop sent"),
                Err(err) => {
                    eprintln!("loop err: {err}");
                    return;
                }
            }
            match handler.send_data(COOLDOWN_CHARA, &[cooldown]).await {
                Ok(_) => println!("cooldown sent"),
                Err(err) => {
                    eprintln!("cooldown err: {err}");
                    return;
                }
            }
            let data = [(duration & 255) as u8, (duration >> 8) as u8];
            match handler.send_data(SHOCK_FLAG, &data).await {
                Ok(_) => println!("data sent"),
                Err(err) => eprintln!("While sending data: {err}"),
            }
            res = Ok(());
        } else {
            eprintln!("Device disconnected prematurely!"); // this is a straight up lie
            *state.lock().await = None;
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

async fn check_connected(app: &AppHandle) -> bool {
    let mutex = app.state::<Mutex<Option<String>>>();
    let mut cur_state = mutex.lock().await;
    if let None = *cur_state {
        *cur_state = scan().await;
    }
    cur_state.is_some()
}

#[tauri::command]
pub fn init_scanloop(app: AppHandle) {
    thread::spawn(move || {
        async_runtime::block_on((|| async {
            loop {
                println!("Checking...");
                if check_connected(&app).await {
                    println!("Emitting event...");
                    if let Err(err) = app.emit("clock_found", ()) {
                        eprintln!("Failed sending event: {err}");
                    }
                    return;
                }
                thread::sleep(Duration::from_secs(10));
            }
        })())
    });
}
