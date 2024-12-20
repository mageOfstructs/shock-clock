// Orthographietrainer Password: 4172

//use shock_clock_ui::components::Home;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use shock_clock_utils::Block;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

use icondata as i;
use icondata_core::IconData;
use leptos::*;
use leptos_icons::*;
use leptos_mview::mview;
use shock_clock_ui::components::{Home, Theme, Watcher};
use shock_clock_ui::{BlocksRS, BlocksWS};

use shock_clock_ui::invoke_without_args;

#[derive(Clone, PartialEq, Copy)]
enum SelectedRoute {
    Watcher,
    Home,
    Themes, // maybe change to settings later
}

impl Display for SelectedRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Watcher => "Watcher",
                Self::Home => "Home",
                Self::Themes => "Themes",
            }
        )
    }
}

#[derive(Clone)]
struct Route(RwSignal<SelectedRoute>);

#[component]
pub fn App() -> impl IntoView {
    spawn_local((|| async {
        invoke_without_args("init_accessibility").await;
        invoke_without_args("init_scanloop").await;
    })());

    let selected_route = RwSignal::new(SelectedRoute::Home);
    provide_context(Route(selected_route));

    let (blocks, set_blocks) = create_signal(Vec::new());
    provide_context(BlocksRS(blocks));
    provide_context(BlocksWS(set_blocks));
    let (selected_theme, set_selected_theme) = create_signal("night".to_string());

    mview! {
        input type="checkbox" value={move || selected_theme()} checked class="checkbox theme-controller hidden"()
        {move || match selected_route() {
            SelectedRoute::Home => mview! {Home()},
            SelectedRoute::Watcher => mview! {Watcher()},
            SelectedRoute::Themes => mview! {Theme set_theme={set_selected_theme}()}
        }}
        div class="btm-nav btm-nav-sm h-[10%]" {
            BtmNavItem route={SelectedRoute::Watcher} icon={i::AiMonitorOutlined}()
            BtmNavItem route={SelectedRoute::Home} icon={i::AiHomeOutlined}()
            BtmNavItem route={SelectedRoute::Themes} icon={i::CgColorBucket}()
        }
    }
}

#[component]
fn BtmNavItem(route: SelectedRoute, icon: &'static IconData) -> impl IntoView {
    let ssr = use_context::<Route>().unwrap().0;
    let moved_route = route.clone(); // no idea why it wants two copies, with normal view! I only
                                     // need one
    mview! {
        button on:click={move |_| ssr.set(moved_route.clone())} class={move || format!("text-primary text-4xl {}", if moved_route == ssr.get() {"active"} else {""})} {
            Icon icon={icon}()
            span class="btm-nav-label"({route.to_string()})
        }
    }
}
