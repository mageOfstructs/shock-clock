// Orthographietrainer Password: 4172

//use shock_clock_ui::components::Home;
use std::fmt::Display;

use icondata as i;
use icondata_core::IconData;
use leptos::*;
use leptos_icons::*;
use leptos_mview::mview;
use shock_clock_ui::components::{Games, Home, Watcher};

#[derive(Clone, PartialEq, Copy)]
enum SelectedRoute {
    Watcher,
    Home,
    Games, // maybe change to settings later
}

impl Display for SelectedRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Watcher => "Watcher",
                Self::Home => "Home",
                Self::Games => "Games",
            }
        )
    }
}

#[derive(Clone)]
struct Route(RwSignal<SelectedRoute>);

#[component]
pub fn App() -> impl IntoView {
    let selected_route = RwSignal::new(SelectedRoute::Watcher);
    provide_context(Route(selected_route));

    mview! {
        {move || match selected_route() {
            SelectedRoute::Home => mview! {Home()},
            SelectedRoute::Watcher => mview! {Watcher()},
            SelectedRoute::Games => mview! {Games()}
        }}
        div class="btm-nav btm-nav-sm h-[10%]" {
            BtmNavItem route={SelectedRoute::Watcher} icon={i::AiMonitorOutlined}()
            BtmNavItem route={SelectedRoute::Home} icon={i::AiHomeOutlined}()
            BtmNavItem route={SelectedRoute::Games} icon={i::CgGames}()
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
