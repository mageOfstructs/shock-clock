use leptos::create_effect;
use leptos::html;
use leptos::logging;
use leptos::provide_context;
use leptos::spawn_local;
use leptos::use_context;
use leptos::view;
use leptos::Children;
use leptos::Effect;
use leptos::NodeRef;
use leptos::Signal;
use leptos::SignalUpdate;
use leptos::SignalWith;
use shock_clock_utils::AppBlockData;
use shock_clock_utils::BlockType;
use shock_clock_utils::ShockStrength;
use shock_clock_utils::WebsiteBlockData;
use std::fmt::Display;
use std::fmt::Formatter;
use uuid;

use icondata as i;
use leptos::For;
use leptos_icons::*;

use leptos::component;
use leptos::create_signal;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos::WriteSignal;
use leptos_mview::mview;
use serde::{Deserialize, Serialize};
use shock_clock_utils::Block;

use serde_wasm_bindgen::to_value;

use super::invoke;

#[derive(Deserialize, Serialize)]
struct BlockArgs {
    blocks: Vec<Block>,
}

async fn update_block_data(blocks: Vec<Block>) {
    invoke(
        "update_blocklist",
        to_value(&BlockArgs { blocks }).expect("real bad"),
    )
    .await;
}

#[derive(Clone, Copy, PartialEq)]
enum WatcherRoute {
    Blacklist,
    Whitelist,
}

impl Display for WatcherRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WatcherRoute::Blacklist => write!(f, "Blacklist"),
            WatcherRoute::Whitelist => write!(f, "Whitelist"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BlockTypeRoute {
    All,
    App,
    Website,
    Keyword,
}

impl Display for BlockTypeRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            BlockTypeRoute::All => write!(f, "All"),
            BlockTypeRoute::App => write!(f, "App"),
            BlockTypeRoute::Website => write!(f, "Website"),
            BlockTypeRoute::Keyword => write!(f, "Keyword"),
        }
    }
}

fn compBTs(btr: BlockTypeRoute, bt: &BlockType) -> bool {
    match btr {
        BlockTypeRoute::All => false,
        BlockTypeRoute::App => {
            if let BlockType::App(_) = bt {
                true
            } else {
                false
            }
        }
        BlockTypeRoute::Website => {
            if let BlockType::Website(_) = bt {
                true
            } else {
                false
            }
        }
        BlockTypeRoute::Keyword => {
            if let BlockType::Keyword = bt {
                true
            } else {
                false
            }
        }
    }
}

#[derive(Clone)]
struct BlocksWS(WriteSignal<Vec<Block>>);

#[derive(Clone)]
struct SSCSignal(ReadSignal<Option<usize>>, WriteSignal<Option<usize>>);

#[component]
pub fn Watcher() -> impl IntoView {
    let (route, set_route) = create_signal(WatcherRoute::Blacklist);
    let (block_type, set_block_type) = create_signal(BlockTypeRoute::All);

    let (blocks, set_blocks) = create_signal(Vec::new());
    provide_context(BlocksWS(set_blocks));

    let filtered_blocks = move || {
        let res: Vec<(usize, Block)> = if let BlockTypeRoute::All = block_type() {
            blocks().into_iter().enumerate().collect()
        } else {
            let btr = block_type();
            blocks()
                .into_iter()
                .filter(|block| compBTs(btr, &block.block_type))
                .enumerate()
                .collect()
        };
        res
    };

    let add_block = move |block: Block| set_blocks.update(|blocks| blocks.push(block));

    let (select_modal_is_open, set_select_modal_is_open) = create_signal(false);
    let (add_modal_is_open, set_add_modal_is_open) = create_signal(false);
    let (add_modal_block_type, set_add_modal_block_type) = create_signal(BlockAdd::App);
    let (shockstren_diag_open, set_shockstren_diag_open) = create_signal(Option::<usize>::None);
    let modal_condition = Signal::derive(move || shockstren_diag_open().is_some());
    provide_context(SSCSignal(shockstren_diag_open, set_shockstren_diag_open));
    let check_condition = move |ss: ShockStrength| {
        let mut ret = false;
        if let Some(i) = shockstren_diag_open() {
            blocks.with(|blocks| ret = blocks[i].shock_strength == ss);
        }
        ret
    };

    add_block(Block {
        uuid: uuid::Uuid::new_v4(),
        name: "Tiktok".to_string(),
        shock_strength: ShockStrength::Normal,
        block_type: BlockType::App(AppBlockData {
            package_name: "com.android.chrome".to_string(),
        }),
    });
    add_block(Block {
        uuid: uuid::Uuid::new_v4(),
        name: "Google".to_string(),
        shock_strength: ShockStrength::Normal,
        block_type: BlockType::Website(WebsiteBlockData {
            url: "www.google.com".to_string(),
        }),
    });
    add_block(Block {
        uuid: uuid::Uuid::new_v4(),
        name: "Videos".to_string(),
        shock_strength: ShockStrength::Normal,
        block_type: BlockType::Keyword,
    });

    Effect::new(move |_| {
        logging::log!("yeah async");
        let cloned_blocks = blocks();
        spawn_local(async move {
            update_block_data(cloned_blocks).await;
        });
    });

    let log = move || {
        format!(
            "WatcherRoute: {}\nBlockTypeRoute: {}\n\n",
            route(),
            block_type()
        )
    };

    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let name_input_ref: NodeRef<html::Input> = NodeRef::new();
    let create_block = move |ba: BlockAdd| {
        let bt = match ba {
            BlockAdd::App => BlockType::App(AppBlockData {
                package_name: input_ref.get().unwrap().value(),
            }),
            BlockAdd::Website => BlockType::Website(WebsiteBlockData {
                url: input_ref.get().unwrap().value(),
            }),
            BlockAdd::Keyword => BlockType::Keyword,
        };
        let name = if ba == BlockAdd::Keyword {
            input_ref.get().unwrap().value()
        } else {
            name_input_ref.get().unwrap().value()
        };
        add_block(Block {
            uuid: uuid::Uuid::new_v4(),
            name,
            shock_strength: ShockStrength::Normal,
            block_type: bt,
        });
    };

    mview! {
        div class="sticky top-0 z-50 bg-base-100 pb-3 pt-3" {
            div class="join flex mx-5" {
                RadioOption value={WatcherRoute::Blacklist} set_signal={set_route} route={route} btn_size="" name="list"()
                RadioOption value={WatcherRoute::Whitelist} set_signal={set_route} route={route} btn_size="" name="list"()
            }

            div class="join flex mx-5 mt-3" {
                RadioOption value={BlockTypeRoute::All} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
                RadioOption value={BlockTypeRoute::App} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
                RadioOption value={BlockTypeRoute::Website} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
                RadioOption value={BlockTypeRoute::Keyword} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
            }
        }
        // p({move || log()})

        // button on:click={move |_| {
        //     add_block(Block {
        //         uuid: uuid::Uuid::new_v4(),
        //         name: "App".to_string(),
        //         shock_strength: ShockStrength::Normal,
        //         block_type: BlockType::App(AppBlockData {
        //             package_name: "com.musically.smth".to_string(),
        //         }),
        //     });
        // }}("Add smth")

        div class="flex justify-center align-center mt-4" {
            button class="btn btn-primary self-center px-8" on:click={move |_| set_select_modal_is_open(true)}("Add Block")
        }

        div class="overflow-y-auto pb-20" {
            ul class="divide-y divide-gray-200" {
                For
                    each={move || filtered_blocks()}
                    key={|(_, block)| block.uuid}
                    children={move |(i, block)| mview! {
                        BlockElement i={i} {block}()
                    }}()
            }
        }

        BlockTypeSelectModal set_block_add_type={set_add_modal_block_type} is_open={select_modal_is_open} set_is_open={set_select_modal_is_open} set_add_modal_open={set_add_modal_is_open}()
        BlockAddModal is_open={add_modal_is_open.into()} {
            button class="btn btn-md btn-circle btn-ghost absolute right-2 top-2" on:click={move |_| set_add_modal_is_open(false)}("X")
            {move || {
                let heading = format!("Block {} {}", if add_modal_block_type() == BlockAdd::App {"an"} else {"a"}, add_modal_block_type());
                view! {
                    <h2>{heading}</h2>
                    <form on:submit={move |ev| {
                        ev.prevent_default();
                        set_add_modal_is_open(false);
                        create_block(add_modal_block_type())
                    }}>
                        {
                            if add_modal_block_type() != BlockAdd::Keyword {
                                view! {
                                    <input type="text" placeholder="Name" node_ref={name_input_ref} required/>
                                }.into_view()
                            } else {
                                view!{}.into_view()
                            }
                        }
                        <input type="text" placeholder="Identifier" node_ref={input_ref} required/>
                        <input type="submit" value="Create"/>
                    </form>
                }
            }}
        }
        // BlockAddModal is_open={modal_condition} {
        //     // button class="btn btn-md btn-circle btn-ghost relative right-2" on:click={move |_| set_shockstren_diag_open(None)}("X")
        //
        //     div class="form-control" {
        //         label class="label cursor-pointer m-4" {
        //             span class="label-text" { "Normal" }
        //             SSCRadioButton checked_condition={check_condition} shockstren={ShockStrength::Normal};
        //         }
        //         label class="label cursor-pointer m-4" {
        //             span class="label-text" { "Hard" }
        //             SSCRadioButton checked_condition={check_condition} shockstren={ShockStrength::Hard};
        //         }
        //         label class="label cursor-pointer m-4" {
        //             span class="label-text" { "Ultra" }
        //             SSCRadioButton checked_condition={check_condition} shockstren={ShockStrength::Ultra};
        //         }
        //     }
        // }
    }
}

#[derive(Clone, PartialEq)]
enum BlockAdd {
    App,
    Website,
    Keyword,
}

// impl Into<BlockType> for BlockAdd {
//     fn into(self) -> T {
//         match self {
//             Self::App => BlockType::App(_);
//         }
//     }
// }

impl Display for BlockAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::App => "App",
                Self::Website => "Website",
                Self::Keyword => "Keyword",
            }
        )
    }
}

#[component]
fn RadioOption<T>(
    value: T,
    set_signal: WriteSignal<T>,
    route: ReadSignal<T>,
    btn_size: &'static str,
    name: &'static str,
) -> impl IntoView
where
    T: Clone + Copy + PartialEq + Display + 'static,
{
    mview! {
        input
            class={move || format!("btn {} join-item flex-1 rounded-l-lg", btn_size)}
            on:click={move |_| set_signal(value)}
            type="radio"
            name={name}
            aria-label={value.to_string()}
            checked={move || route() == value}()
    }
}

#[component]
fn BlockElement(i: usize, block: Block) -> impl IntoView {
    let set_blocks = use_context::<BlocksWS>().unwrap().0;
    let set_ssci = use_context::<SSCSignal>().unwrap().1;
    let remove_block = move |uuid: uuid::Uuid| {
        set_blocks.update(|blocks| blocks.retain(|block| block.uuid != uuid))
    };

    mview! {
        li class="flex p-4" {
            div class="flex flex-auto w-1/2 items-start space-x-3" {
                {match &block.block_type {
                    BlockType::App(_) => mview!{ Icon width="3em" height="3em" icon={i::AiAppstoreOutlined}() },
                    BlockType::Website(_) => mview!{ Icon width="3em" height="3em" icon={i::MdiWeb}() },
                    BlockType::Keyword => mview!{ Icon width="3em" height="3em" icon={i::BsCardText}() }
                }}
                div {
                    span class="text-primary text-2xl"({block.name})
                    p class="text-sm text-gray-400"({move || match &block.block_type {
                        BlockType::App(ref app_data) => app_data.package_name.clone(),
                        BlockType::Website(ref website_data) => website_data.url.clone(),
                        _ => "".to_string()
                    }})
                }
            }
            div class="flex justify-end flex-auto w-1/2" {
                // komischer ShockStrength button
                // button class="btn btn-warning" on:click={move |_| set_ssci(Some(i))} {
                //     Icon width="2em" height="2em" icon={i::BsLightningCharge}()
                // }
                // Delete Button
                button class="btn btn-error" on:click={move |_| remove_block(block.uuid)} {
                    Icon width="2em" height="2em" icon={i::BsTrash}()
                }
            }
        }
    }
}

#[component]
fn BlockTypeSelectModal(
    set_block_add_type: WriteSignal<BlockAdd>,
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
    set_add_modal_open: WriteSignal<bool>,
) -> impl IntoView {
    mview! {
        dialog class={move || format!("modal {}", if is_open() {"modal-open"} else {""})} {
            div class="modal-box flex flex-col" {
                button class="btn btn-md btn-circle btn-ghost absolute right-2 top-2" on:click={move |_| set_is_open(false)}("X")
                button class="btn btn-secondary btn-outline mt-8" on:click={move |_| {
                    set_is_open(false);
                    set_block_add_type(BlockAdd::App);
                    set_add_modal_open(true);
                }} ("App")
                button class="btn btn-secondary btn-outline mt-2" on:click={move |_| {
                    set_is_open(false);
                    set_block_add_type(BlockAdd::Website);
                    set_add_modal_open(true);
                }} ("Website")
                button class="btn btn-secondary btn-outline mt-2"  on:click={move |_| {
                    set_is_open(false);
                    set_block_add_type(BlockAdd::Keyword);
                    set_add_modal_open(true);
                }} ("Keyword")
            }
        }
    }
}

#[component]
fn BlockAddModal(is_open: Signal<bool>, children: Children) -> impl IntoView {
    mview! {
        dialog class={move || format!("modal {}", if is_open() {"modal-open"} else {""})} {
            div class="modal-box" {
                {children()}
            }
        }
    }
}

fn change_shock_strength(set_blocks: WriteSignal<Vec<Block>>, i: usize, shockstren: ShockStrength) {
    set_blocks.update(|blocks| blocks[i].shock_strength = shockstren)
}

// #[component]
// fn SSCRadioButton(
//     checked_condition: impl Fn(ShockStrength) -> bool + 'static,
//     shockstren: ShockStrength,
// ) -> impl IntoView {
//     let newtype = use_context::<SSCSignal>().unwrap();
//     let shockstren_diag_open = newtype.0;
//     let set_shockstren_diag_open = newtype.1;
//
//     let set_blocks = use_context::<BlocksWS>().unwrap().0;
//     mview! {
//             {move || if checked_condition(shockstren) {
//                 mview! {
//                     input type="radio" name="radio-10" class="radio checked:bg-red-500" checked="checked" on:click={move |_| {
//         change_shock_strength(set_blocks, shockstren_diag_open().unwrap(), shockstren);
//         set_shockstren_diag_open(None);
//     }};
//                 }
//             } else {
//                 mview! {
//                     input type="radio" name="radio-10" class="radio checked:bg-red-500" on:click={move |_| {
//         change_shock_strength(set_blocks, shockstren_diag_open().unwrap(), shockstren);
//         set_shockstren_diag_open(None);
//     }};
//                 }
//             }
//         }
//     }
// }
