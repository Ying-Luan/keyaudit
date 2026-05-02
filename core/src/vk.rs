/// Converts a virtual key code to a human-readable string.
///
/// # Arguments
///
/// * `vk` - The virtual key code to convert.
///
/// # Returns
///
/// A string representation of the virtual key code.
pub fn vk_to_string(vk: u32) -> String {
    match vk {
        0x08 => "Backspace",
        0x09 => "Tab",
        0x0D => "Enter",
        0x1B => "Esc",
        0x20 => "Space",
        0x21 => "PgUp",
        0x22 => "PgDn",
        0x23 => "End",
        0x24 => "Home",
        0x2D => "Ins",
        0x2E => "Del",
        0x30..=0x39 => return format!("{}", vk as u8 as char), // 0-9
        0x41..=0x5A => return format!("{}", vk as u8 as char), // A-Z
        0x70 => "F1",
        0x71 => "F2",
        0x72 => "F3",
        0x73 => "F4",
        0x74 => "F5",
        0x75 => "F6",
        0x76 => "F7",
        0x77 => "F8",
        0x78 => "F9",
        0x79 => "F10",
        0x7A => "F11",
        0x7B => "F12",
        0xBB => "=",
        0xBD => "-",
        0xBF => "/",
        0xC0 => "`",
        0xDB => "[",
        0xDD => "]",
        0xDC => "\\",
        0xDE => "'",
        0xBC => ",",
        0xBE => ".",
        0xBA => ";",
        _ => return format!("VK_{:#04X}", vk),
    }
    .to_string()
}

pub fn common_vk_range() -> Vec<u32> {
    (0x30..=0x39) // 0-9
        .chain(0x41..=0x5A) // A-Z
        .chain(0x70..=0x7B) // F1-F12
        .chain([
            0x09, 0x0D, 0x1B, 0x20, 0x2D, 0x2E, 0xBB, 0xBD, 0xBF, 0xC0, 0xDB, 0xDD, 0xDC, 0xDE,
            0xBC, 0xBE, 0xBA,
        ])
        .collect()
}
