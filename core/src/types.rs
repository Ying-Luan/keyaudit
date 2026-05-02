use serde::{Deserialize, Serialize};

use crate::modifier::ModFlags;

#[derive(Debug, Serialize, Deserialize)]
pub enum HotKeyStatus {
    Occupied,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotKey {
    pub modifiers: Vec<String>,
    pub mod_flags: ModFlags,
    pub key: String,
    pub vk_code: u32,
    /// The status of the hotkey.
    pub status: HotKeyStatus,
}
