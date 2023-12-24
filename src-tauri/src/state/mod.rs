use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub running_service: Arc<Mutex<RunningService>>,
}

#[derive(Default)]
pub struct RunningService {
    list: HashMap<String, u32>,
}

impl RunningService {
    pub fn add(&mut self, name: String, pid: u32) -> &Self {
        self.list.insert(name, pid);
        return self;
    }

    pub fn remove(&mut self, name: &String) -> &Self {
        self.list.remove(name);
        self
    }

    pub fn is_running(&self, name: &String) -> bool {
        self.list.contains_key(name)
    }

    pub fn get_pid(&self, name: &String) -> Option<String> {
        return match self.list.get(name) {
            Some(pid) => Some(pid.to_string()),
            None => None,
        };
    }

    pub fn running(&self) -> HashMap<String, u32> {
        self.list.clone()
    }
}
