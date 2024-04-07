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
    pub fn create_task(& mut self, cfg:task::TaskConfig) -> u64 {
        let nt = task::Task::from_config(cfg);
        let id = nt.id;
        println!("create task id:{}, title:{}", nt.id, nt.title);
        self.tasks.push(nt);
        for ele in &self.tasks {
            println!("curr task id:{}, title:{}", ele.id, ele.title);
        }
        return id
    }

    pub fn get_task(& mut self, id: u64) -> Option<task::Task> {
        for ele in &self.tasks {
            if ele.id == id {
                println!("get task id:{}, title:{}", ele.id, ele.title);
                return Some(ele.clone());
            }
        }
        println!("get task id:{} not found", id);
        return None
    }
    
    pub fn list_task(& mut self) -> Vec<task::Task> {
        return self.tasks.clone();
    }
}

pub struct ServerState(Mutex<Server>);


#[tauri::command]
fn create_task(state: tauri::State<ServerState>, t:task::TaskConfig) -> u64 {
    let mut s = state.0.lock().unwrap();
    println!("{}", t.title);
    return s.create_task(t);
}

#[tauri::command]
fn get_task(state: tauri::State<ServerState>, id: u64) -> Option<task::Task> {
    let mut s = state.0.lock().unwrap();
    return s.get_task(id);
}

#[tauri::command]
fn list_task(state: tauri::State<ServerState>) -> Vec<task::Task> {
    let mut s = state.0.lock().unwrap();
    return s.list_task();
}

fn main() {
    tauri::Builder::default()
        .manage(ServerState(Default::default()))
        .invoke_handler(tauri::generate_handler![next_number, create_task, get_task,list_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
