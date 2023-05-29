// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// The function greet only greets Leo, if you send a different name it error
#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name.eq("Leo") {
        return Ok(format!("Hello, {}! You've been greeted from Rust!", name));
    } 
    Err(format!("You are not allowed here {}, only Leo is allowed", name))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
