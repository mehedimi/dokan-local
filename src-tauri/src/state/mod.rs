use std::collections::HashMap;
use std::process::Command;
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

    pub fn close_by_pid(&self, pid: &str) -> bool {
        let child = Command::new("kill").args(["-s", "TERM", pid]).spawn();

        if child.is_err() {
            return false;
        }

        let child = child.unwrap().wait();

        if child.is_err() {
            return false;
        }

        child.unwrap().success()
    }

    pub fn close(&self, service: &String) -> bool {
        match self.list.get(service) {
            Some(pid) => self.close_by_pid(pid.to_string().as_str()),
            None => false,
        }
    }

    pub fn close_all(&self) {
        for (_, pid) in self.list.clone().into_iter() {
            self.close_by_pid(pid.to_string().as_str());
        }
    }
}
