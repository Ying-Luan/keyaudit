use windows::Win32::UI::Input::KeyboardAndMouse::{
    HOT_KEY_MODIFIERS, RegisterHotKey, UnregisterHotKey,
};

use crate::{
    modifier::MOD_COMBOS,
    scanner::Scanner,
    types::{HotKey, HotKeyStatus},
    vk::{common_vk_range, vk_to_string},
};

pub struct WindowsScanner;

impl Scanner for WindowsScanner {
    fn scan(&self) -> Vec<HotKey> {
        let vk_range = common_vk_range();
        let mut results = Vec::new();
        let id = 0xA000u32;

        for &mod_flags in MOD_COMBOS {
            for &vk in &vk_range {
                let ok = unsafe {
                    RegisterHotKey(None, id as i32, HOT_KEY_MODIFIERS(mod_flags.raw()), vk).is_ok()
                };
                if ok {
                    unsafe {
                        UnregisterHotKey(None, id as i32).ok();
                    }
                    continue;
                }
                results.push(HotKey {
                    modifiers: mod_flags.into(),
                    mod_flags,
                    key: vk_to_string(vk),
                    vk_code: vk,
                    status: HotKeyStatus::Occupied,
                });
            }
        }

        results
    }
}
