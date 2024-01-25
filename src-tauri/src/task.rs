use rand::Rng;
use chrono::prelude::*;

pub enum State {
    Pending,
    Doing,
    Paused,
    Canceld,
    Done,
    Failed,

}

pub struct Task {
    pub id: u64,
    pub title: String,
    pub create_time: DateTime<chrono::Local>,
    pub last_update_time: Option<DateTime<chrono::Local>>,
    pub childs: Vec<u64>,
    pub state: State,
}

pub fn new_task(title: String) -> Task {
    return Task {
        id: alloc_id(),
        title: title,
        create_time: Local::now(),
        last_update_time: None,
        childs: Vec::new(),
        state: State::Pending }
}

fn alloc_id() -> u64 {
    return rand::thread_rng().gen_range(1..101);
}