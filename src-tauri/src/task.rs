use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    Pending,
    Doing,
    Paused,
    Canceld,
    Done,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskConfig {
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub create_time: Option<DateTime<Local>>,
    pub last_update_time: Option<DateTime<Local>>,
    pub childs: Vec<u64>,
    pub state: State,
}

pub fn new(cfg: TaskConfig) -> Task {
    return Task {
        id: alloc_id(),
        title: cfg.title,
        create_time: Some(Local::now()),
        last_update_time: Some(Local::now()),
        childs: Vec::new(),
        state: State::Pending
    }
}

fn alloc_id() -> u64 {
    return rand::thread_rng().gen_range(1..101);
}
