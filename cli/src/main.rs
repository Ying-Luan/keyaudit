use keyaudit_core::{PlatformScanner, Scanner};

fn main() {
    let scanner = PlatformScanner;
    let hotkeys = scanner.scan();

    if hotkeys.is_empty() {
        println!("No occupied hotkeys found.");
        return;
    }

    #[cfg(debug_assertions)]
    println!("{:#?}", hotkeys);

    #[cfg(not(debug_assertions))]
    for hk in &hotkeys {
        let combo = format!("{}+{}", hk.modifiers.join("+"), hk.key);
        println!("{}", combo);
    }
}
