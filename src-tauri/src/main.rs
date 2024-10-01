// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn sayhelo(name: &str) -> String {
    format!("helo to {} from AnkaOS!!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sayhelo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
