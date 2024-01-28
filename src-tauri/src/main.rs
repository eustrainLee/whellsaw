// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod task;

use std::{sync::Mutex, vec::Vec};

#[tauri::command]
fn next_number(value: Option<i32>) -> i32 {
    match value {
    Some(num) => num+1,
    None => 0,
    }
}

#[derive(Default)]
struct Server {
    tasks: Vec::<task::Task>
}

impl Server {
    pub fn new_task(& mut self, cfg:task::TaskConfig) -> u64 {
        let nt = task::new(cfg);
        let id = nt.id;
        println!("new task id:{}, title:{}", nt.id, nt.title);
        self.tasks.push(nt);
        for ele in &self.tasks {
            println!("curr task id:{}, title:{}", ele.id, ele.title);
        }
        return id
    }
}

pub struct ServerState(Mutex<Server>);


#[tauri::command]
fn new_task(state: tauri::State<ServerState>, t:task::TaskConfig) -> u64 {
    let mut s = state.0.lock().unwrap();
    println!("{}", t.title);
    return s.new_task(t);
}

fn main() {
    tauri::Builder::default()
        .manage(ServerState(Default::default()))
        .invoke_handler(tauri::generate_handler![next_number, new_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
