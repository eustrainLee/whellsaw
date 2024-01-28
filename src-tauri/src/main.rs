// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod task;

use std::{sync::Mutex, vec::Vec};

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

#[derive(Default)]
struct Server {
    tasks: Vec::<task::Task>
}

impl Server {
    pub fn new_task(& mut self, cfg:task::TaskConfig) {
        let nt = task::new(cfg);
        println!("new task id:{}, title:{}", nt.id, nt.title);
        self.tasks.push(nt);
        for ele in &self.tasks {
            println!("curr task id:{}, title:{}", ele.id, ele.title);
        }
    }
}

pub struct ServerState(Mutex<Server>);


#[tauri::command]
fn new_task(state: tauri::State<ServerState>, t:task::TaskConfig) {
    let mut s = state.0.lock().unwrap();
    println!("{}", t.title);
    s.new_task(t);
    
}

fn main() {
    
    tauri::Builder::default()
        .manage(ServerState(Default::default()))
        .invoke_handler(tauri::generate_handler![next_number,refresh, new_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
