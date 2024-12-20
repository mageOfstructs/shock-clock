#![feature(async_closure)]
use leptos::{ReadSignal, WriteSignal};
use shock_clock_utils::Block;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    // They need to have different names!
}

#[derive(Clone)]
pub struct BlocksWS(pub WriteSignal<Vec<Block>>);

#[derive(Clone)]
pub struct BlocksRS(pub ReadSignal<Vec<Block>>);

pub mod components;
