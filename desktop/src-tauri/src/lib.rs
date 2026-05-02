use keyaudit_core::{HotKey, PlatformScanner, Scanner};

#[tauri::command]
fn scan() -> Vec<HotKey> {
    PlatformScanner.scan()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
