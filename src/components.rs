use leptos::ev::MouseEvent;
use leptos::logging;
use leptos::use_context;
use leptos::view;
use leptos::Await;
use leptos::SignalGet;
use leptos::SignalUpdate;
use leptos::WriteSignal;
use shock_clock_utils::ble::IsConnected;
use shock_clock_utils::Block;

use icondata as i;
use leptos::component;
use leptos::create_signal;
use leptos::IntoView;
use leptos_icons::Icon;
use leptos_mview::mview;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use tauri_sys::event;

use crate::BlocksWS;

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

pub async fn wait_for_addr(set_clock_connected: WriteSignal<bool>) -> bool {
    if from_value(invoke_without_args("is_connected").await).expect("JsValue(null)") {
        set_clock_connected(true);
        return true;
    }
    let res: Result<event::Event<()>, tauri_sys::Error> = event::once("clock_found").await;
    match res {
        Ok(_) => {
            set_clock_connected(true);
            true
        }
        Err(err) => {
            logging::error!("Error: {err}");
            false
        }
    }
}

#[derive(Clone)]
struct ToggleStates {
    social_media: bool,
    gambling: bool,
    adult_content: bool,
}

#[component]
pub fn Home() -> impl IntoView {
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

    let (clock_connected, set_clock_connected) = create_signal(false);
    let (clock_stat, set_clock_stat) = create_signal(false);
    let set_blocks = use_context::<BlocksWS>().unwrap().0;
    let (ts, set_ts) = create_signal(ToggleStates {
        social_media: false,
        gambling: false,
        adult_content: false,
    });
    let get_icon = move || {
        if clock_stat() {
            i::AiThunderboltFilled
        } else {
            i::AiWarningFilled
        }
    };
    mview! {
        div class="prose h-screen" {
            h1 class="my-4" { "Shock Clock" }
            div class="flex flex-col h-4/5 my-6" {
                div class="stats" {
                    div class="stat" {
                        span class="stat-title" { "Watcher" }
                        Icon icon={i::AiThunderboltFilled};
                    }
                    div class="stat" {
                        span class="stat-title" { "Clock" }
                        Await
                            future={move || wait_for_addr(set_clock_connected)}
                            |_| {
                                Icon icon={i::AiThunderboltFilled};
                            }
                        {move || if !clock_connected() {
                            mview!{
                                Icon icon={i::AiWarningFilled};
                            }.into_view()
                            } else {
                            mview!{
                            }.into_view()
                                }
                        }
                    }
                }

                div class="flex flex-auto h-6/16 pt-48" {
                    div class="flex-1";
                    button on:click={shock_test} class="btn center text-6xl rounded-full flex-auto h-4/6 w-1/12 border-yellow-500 border-4 hover:border-white clicked:border-yellow-500" {"⚡"}
                    div class="flex-1";
                }
                div class="flex-1 form-control" style="visibility: hidden" {
                    Toggle label="Block Social Media" callback={move |_| {
                    // set_blocks.update(|block| {
                    //     if ts().social_media {
                    //         block.push(Block::new("TikTok", ));
                    //         block.push(Block::new("YouTube", ));
                    //         block.push(Block::new("Instagram", ));
                    //         block.push(Block::new("Facebook", ));
                    //     }
                    // });
                }};
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

// #[component]
// pub fn Themes() -> impl IntoView {
//     mview! {
//         h1 class="my-4" { "Credits" }
//         p { "made with blood, sweat, tears and move closures by"}
//         ul {
//             li {
//                 a href="https://github.com/mageofstructs" {"@mageOfStructs (Jason)"}
//             }
//             li {
//                 a href="https://github.com/thronkatze0" {"@thronkatze0 (Vincent)"}
//             }
//         }
//         h3 { "Special Thanks" }
//         ul {
//             li {"Linus Torvalds - Creator of Linux"}
//             li {"Ken Thompson - Co-Creator of UNIX"}
//             li {"Dennis Richie - Creator of C/Co-Creator of UNIX"}
//             li {"Bjarne Stroustrup - Creator of C++"}
//         }
//     }
// }

#[component]
pub fn Theme(set_theme: WriteSignal<String>) -> impl IntoView {
    let themes = [
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
    ];

    mview! {
        div class="grid grid-cols-3 gap-4 p-4" {
            {themes.into_iter()
            .map(|theme| mview! {
                button class="btn btn-secondary btn-outline" on:click={move |_| set_theme(theme.to_string())} ({theme.to_string()})
            })
            .collect::<Vec<_>>()}
        }
    }
}
