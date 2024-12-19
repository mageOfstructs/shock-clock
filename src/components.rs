use leptos::ev::MouseEvent;
use leptos::logging;
use leptos::view;
use leptos::Await;
use leptos::WriteSignal;
use shock_clock_utils::ble::IsConnected;

use icondata as i;
use leptos::component;
use leptos::create_signal;
use leptos::IntoView;
use leptos_icons::Icon;
use leptos_mview::mview;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use tauri_sys::event;

pub use super::{invoke, invoke_without_args};

#[derive(Serialize, Deserialize)]
struct ShockArgs {
    duration: u16,
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
    spawn_local((|| async {
        invoke_without_args("init_scanloop").await;
    })());
    let shock_test = move |_| {
        spawn_local((async move || {
            // FIXME: currently still blocks the thread
            logging::log!("shocking...?");
            let args = to_value(&ShockArgs { duration: 500 }).unwrap();
            logging::log!("args seems good...");
            logging::log!("greet works...");
            let res = invoke("shock", args).await;
            println!("{res:?}");
        })());
    };
    let wait_for_addr = || async {
        let res: Result<event::Event<()>, tauri_sys::Error> = event::once("clock_found").await;
        match res {
            Ok(_) => true,
            Err(err) => {
                logging::error!("Error: {err}");
                false
            }
        }
    };
    let (clock_stat, set_clock_stat) = create_signal(false);
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
                        Await
                            future={wait_for_addr}
                            |_| {
                                Icon icon={i::AiCheckOutlined};
                            }
                    }
                }

                div class="flex flex-auto h-6/16 pt-48" {
                    div class="flex-1";
                    button on:click={shock_test} class="btn center text-6xl rounded-full flex-auto h-4/6 w-1/12 border-yellow-500 border-4 hover:border-white clicked:border-yellow-500" {"⚡"}
                    div class="flex-1";
                }
                div class="flex-1 form-control" {
                    Toggle label="Block Social Media" callback={move |_| {}};
                    Toggle label="Block Gambling" callback={move |_| {}};
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
    mview! {
        h1 class="my-4" { "Credits" }
        p { "made with blood, sweat, tears and move closures by"}
        ul {
            li {
                a href="https://github.com/mageofstructs" {"@mageOfStructs (Jason)"}
            }
            li {
                a href="https://github.com/thronkatze0" {"@thronkatze0 (Vincent)"}
            }
        }
        h3 { "Special Thanks" }
        ul {
            li {"Linus Torvalds - Creator of Linux"}
            li {"Ken Thompson - Co-Creator of UNIX"}
            li {"Dennis Richie - Creator of C/Co-Creator of UNIX"}
            li {"Bjarne Stroustrup - Creator of C++"}
        }
    }
}
