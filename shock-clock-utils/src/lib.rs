use serde::{Deserialize, Serialize};
use uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub uuid: uuid::Uuid,
    pub name: String,
    pub shock_strength: ShockStrength,
    pub block_type: BlockType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BlockType {
    App(AppBlockData),
    Website(WebsiteBlockData),
    Keyword,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
pub enum ShockStrength {
    Normal,
    Hard,
    Ultra,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppBlockData {
    pub package_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebsiteBlockData {
    pub url: String,
}

pub mod ble;
