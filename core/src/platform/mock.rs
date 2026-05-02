use crate::{
    modifier::{MOD_ALT, MOD_CTRL, MOD_SHIFT},
    types::{HotKey, HotKeyStatus},
};

pub struct MockScanner;

impl Scanner for MockScanner {
    fn scan(&self) -> Vec<HotKey> {
        vec![
            HotKey {
                modifiers: vec!["Ctrl".into(), "Alt".into()],
                mod_flags: MOD_CTRL | MOD_ALT,
                key: "A".into(),
                vk_code: 0x41,
                status: HotKeyStatus::Occupied,
            },
            HotKey {
                modifiers: vec!["Shift".into()],
                mod_flags: MOD_SHIFT,
                key: "B".into(),
                vk_code: 0x42,
                status: HotKeyStatus::Occupied,
            },
        ]
    }
}
