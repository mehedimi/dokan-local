use crate::state::{AppState, Service};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tauri::Window;

#[derive(Serialize, Clone)]
struct ServiceLog {
    pub service: String,
    pub log: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ServiceStart {
    pub service: String,
    pub p_id: u32,
    pub is_dev: bool
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ServiceStop {
    pub service: String,
}

fn send_log<S: serde::Serialize>(window: &Window, event: &str, payload: S) {
    window
        .emit_and_trigger(event, &payload)
        .expect(&*format!("Failed to send {} log", event))
}

#[tauri::command]
pub fn start_service(
    window: Window,
    state: tauri::State<'_, AppState>,
    root_dir: String,
    port: u32,
    service: String,
    is_dev: bool,
) -> Result<u32, String> {
    // npx ts-node-dev --respawn --transpile-only --exit-child -r tsconfig-paths/register src/index.ts

    if state.running_service.lock().unwrap().is_running(&service) {
        return Err(format!("{} is already been running", &service));
    }

    let program = if &service == "shipping-service" {
        if is_dev {
            "air"
        } else {
            "./tmp/api"
        }
    } else {
        "pnpm"
    };

    let cmd = match is_dev {
        true => "dev",
        false => "start",
    };

    tauri::async_runtime::spawn(async move {
        let mut child = Command::new(program);

        let child = match service.as_str() {
            "storefront" => child.args(["run", &cmd, "-p", "3001"]),
            "dashboard" => child.args(["run", &cmd, "--port", "3000"]),
            "shipping-service" => child.env("APP_PORT", port.to_string()),
            _ => child.args(["run", &cmd]).env("APP_PORT", port.to_string()),
        }
        .current_dir(format!("{}/{}", &root_dir, &service))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

        if child.is_err() {
            send_log(
                &window,
                "service:logs",
                ServiceLog {
                    service: service.clone(),
                    log: child.err().unwrap().to_string(),
                },
            );
            return;
        }

        let mut child = child.unwrap();

        send_log(
            &window,
            "service:start",
            ServiceStart {
                service: service.clone(),
                p_id: child.id(),
                is_dev
            },
        );

        match child.stdout.take() {
            Some(stdout) => {
                for line in BufReader::new(stdout).lines().flatten() {
                    send_log(
                        &window,
                        "service:logs",
                        ServiceLog {
                            service: service.clone(),
                            log: line,
                        },
                    );
                }
            }
            None => {}
        }

        match child.stderr.take() {
            Some(stderr) => {
                for line in BufReader::new(stderr).lines().flatten() {
                    send_log(
                        &window,
                        "service:logs",
                        ServiceLog {
                            service: service.clone(),
                            log: line,
                        },
                    );
                }
            }
            None => {}
        }

        child.wait().expect("Failed to running the service");
    });

    Ok(1)
}

#[tauri::command]
pub fn stop_service(
    service: String,
    state: tauri::State<'_, AppState>,
    window: Window,
) -> Result<String, String> {
    if !state.running_service.lock().unwrap().is_running(&service) {
        return Err(format!("This {} is not running!", service));
    }

    state.running_service.lock().unwrap().close(&service);

    send_log(&window, "service:stop", ServiceStop { service });

    Ok("Done".to_string())
}

#[tauri::command]
pub fn running_service(state: tauri::State<AppState>) -> HashMap<String, Service> {
    state.running_service.lock().unwrap().running()
}

#[tauri::command]
pub async fn git_pull(window: Window, service: String, root_dir: String) -> Result<bool, String> {
    let path = format!("{}/{}", root_dir, &service);

    let branch = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(&path)
        .output()
        .unwrap()
        .stdout;

    let mut child = Command::new("git");
    let child = child
        .arg("pull")
        .arg("origin")
        .arg(String::from_utf8(branch).unwrap().trim_end())
        .current_dir(format!("{}/{}", root_dir, &service))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    if child.is_err() {
        send_log(
            &window,
            "service:logs",
            ServiceLog {
                service: service.clone(),
                log: child.err().unwrap().to_string(),
            },
        );
        return Ok(false);
    }

    let mut child = child.unwrap();

    match child.stdout.take() {
        Some(stdout) => {
            for line in BufReader::new(stdout).lines().flatten() {
                send_log(
                    &window,
                    "service:logs",
                    ServiceLog {
                        service: service.clone(),
                        log: line,
                    },
                );
            }
        }
        None => {}
    }

    child.wait().expect("Git pull error");

    Ok(true)
}

#[tauri::command]
pub async fn service_build(window: Window, service: String, root_dir: String) -> Result<bool, String> {
    let program = if service == "shipping-service" { "go" } else {"pnpm"};

    let mut child = Command::new(program);

    let child = child
        .arg("build");
        if program == "go" {
            child.args(["-o", "tmp/api", "cmd/api/main.go"]);
        }

    let child = child.current_dir(format!("{}/{}", root_dir, &service))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    if child.is_err() {
        send_log(
            &window,
            "service:logs",
            ServiceLog {
                service: service.clone(),
                log: child.err().unwrap().to_string(),
            },
        );
        return Ok(false);
    }

    let mut child = child.unwrap();

    match child.stdout.take() {
        Some(stdout) => {
            for line in BufReader::new(stdout).lines().flatten() {
                send_log(
                    &window,
                    "service:logs",
                    ServiceLog {
                        service: service.clone(),
                        log: line,
                    },
                );
            }
        }
        None => {}
    }

    match child.wait() {
        Ok(_) => {}
        Err(err) => {
            send_log(
                &window,
                "service:logs",
                ServiceLog {
                    service: service.clone(),
                    log: err.to_string(),
                },
            );
        }
    }

    Ok(true)
}
