// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[tauri::command]
fn next_number(value: i32) -> i32 {
  return value+2
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![next_number])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
