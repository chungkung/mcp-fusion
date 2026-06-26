// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "tauri-runtime")]
fn main() {
    tauri_app_lib::run();
}

#[cfg(not(feature = "tauri-runtime"))]
fn main() {
    eprintln!("This binary requires the 'tauri-runtime' feature to run.");
    std::process::exit(1);
}
