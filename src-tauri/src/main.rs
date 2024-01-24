// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[tauri::command]
fn next_number(value: i32) -> i32 {
  return value+1
}

#[tauri::command]
fn refresh(value: String, count: i32) -> (String, i32) {
  if value.len() <= 9 {
    return (value + "abc", count+1)
  } else {
    return (String::from(&value[0..3]),count+1)// String::from(&value[0..3])
  }
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![next_number,refresh])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
