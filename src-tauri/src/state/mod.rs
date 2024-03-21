use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Service {
    pid: u32,
    is_dev: bool,
}

#[derive(Default)]
pub struct AppState {
    pub running_service: Arc<Mutex<RunningService>>,
}

#[derive(Default)]
pub struct RunningService {
    list: HashMap<String, Service>,
}

impl RunningService {
    pub fn add(&mut self, name: String, pid: u32, is_dev: bool) -> &Self {
        self.list.insert(name, Service { pid, is_dev });
        return self;
    }

    pub fn remove(&mut self, name: &String) -> &Self {
        self.list.remove(name);
        self
    }

    pub fn is_running(&self, name: &String) -> bool {
        self.list.contains_key(name)
    }

    pub fn running(&self) -> HashMap<String, Service> {
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
            Some(ser) => self.close_by_pid(ser.pid.to_string().as_str()),
            None => false,
        }
    }

    pub fn close_all(&self) {
        for (_, ser) in self.list.clone().into_iter() {
            self.close_by_pid(ser.pid.to_string().as_str());
        }
    }
}
