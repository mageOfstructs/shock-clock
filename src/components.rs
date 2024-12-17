use leptos::ev::MouseEvent;
use leptos::logging;
use leptos::on_cleanup;
use leptos::set_interval_with_handle;
use leptos::view;
use leptos::Signal;
use leptos::WriteSignal;
use shock_clock_utils::ble::IsConnected;
use std::time::Duration;

use icondata as i;
use leptos::component;
use leptos::create_signal;
use leptos::IntoView;
use leptos_icons::Icon;
use leptos_mview::mview;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct ShockArgs {
    duration: u16,
}

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    // They need to have different names!
}

async fn update_clock_stat(set_clock_stat: WriteSignal<bool>) {
    let is_connected: IsConnected =
        from_value(invoke_without_args("is_connected").await).expect("JsValue(null)");

    set_clock_stat(is_connected.0);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Home() -> impl IntoView {
    let shock_test = move |_| {
        spawn_local((async move || {
            logging::log!("shocking...?");
            let args = to_value(&ShockArgs { duration: 1000 }).unwrap();
            logging::log!("args seems good...");
            logging::log!("greet works...");
            let res = invoke("shock", args).await;
            println!("{res:?}");
        })());
    };
    let (clock_stat, set_clock_stat) = create_signal(false);
    // let handle = set_interval_with_handle(
    //     move || {
    //         spawn_local(update_clock_stat(set_clock_stat));
    //     },
    //     Duration::from_secs(5),
    // );
    // on_cleanup(move || {
    //     drop(handle);
    // });
    let get_icon = move || {
        if clock_stat() {
            i::AiCheckOutlined
        } else {
            i::AiCloseOutlined
        }
    };
    mview! {
        div class="prose h-screen" {
            h1 class="my-4" { "Shock Clock" }
            div class="flex flex-col h-4/5 my-6" {
                div class="stats" {
                    div class="stat" {
                        span class="stat-title" { "Watcher" }
                        Icon icon={i::AiCloseOutlined};
                    }
                    div class="stat" {
                        span class="stat-title" { "Clock" }
                        Icon icon={Signal::derive(get_icon)};
                    }
                }

                div class="flex flex-auto h-6/16 pt-48" {
                    div class="flex-1";
                    button on:click={shock_test} class="btn center text-6xl rounded-full flex-auto h-4/6 w-1/12 border-yellow-500 border-4" {"⚡"}
                    div class="flex-1";
                }
                div class="flex-1 form-control" {
                    Toggle label="Block Social Media" callback={move |_| {}};
                    Toggle label="Block Shorts" callback={move |_| {}};
                    Toggle label="Block Adult Content" callback={move |_| {}};
                }
            }
        }
    }
}

#[component]
fn Toggle(label: &'static str, callback: impl Fn(MouseEvent) -> () + 'static) -> impl IntoView {
    view! { // need to use view! bc other stupid macro can't understand that label param is supposed to
        // be a child
        <label class="mx-4 label cursor-pointer">
            <span class="label-text text-lg">{label}</span>
            <input type="checkbox" class="toggle toggle-lg" on:click={callback}/>
        </label>
    }
}

pub mod watcher;

use wasm_bindgen_futures::spawn_local;
pub use watcher::Watcher;

#[component]
pub fn Games() -> impl IntoView {
    mview! {}
}
