use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{stderr, BufRead, BufReader};
use std::path::Path;
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
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ServiceStop {
    pub service: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ServiceClone {
    pub service: String,
    pub log: String,
}

fn send_log<S: serde::Serialize>(window: &Window, event: &str, payload: S) {
    window
        .emit_and_trigger(event, &payload)
        .expect(&*format!("Failed to send {} log", event))
}

#[tauri::command(async)]
pub async fn start_service(
    window: Window,
    state: tauri::State<'_, AppState>,
    root_dir: String,
    port: u32,
    service: String,
) -> Result<u32, String> {
    // npx ts-node-dev --respawn --transpile-only --exit-child -r tsconfig-paths/register src/index.ts

    if state.running_service.lock().unwrap().is_running(&service) {
        return Err(format!("{} is already been running", &service));
    }

    let program = if &service == "shipping-service" {
        "air"
    } else {
        "pnpm"
    };

    tokio::spawn(async move {
        let mut child = Command::new(program);

        let child = match service.as_str() {
            "storefront" => child.args(["dev", "-p", "3001"]),
            "dashboard" => child.args(["dev", "--port", "3000"]),
            _ => child.arg("dev").env("APP_PORT", port.to_string()),
        }
        .current_dir(format!("{}/{}", &root_dir, &service))
        .stdout(Stdio::piped())
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
            },
        );

        let stdout = child
            .stdout
            .take()
            .expect("Failed to stream output to stdout");

        let reader = BufReader::new(stdout);

        for line in reader.lines().flatten() {
            send_log(
                &window,
                "service:logs",
                ServiceLog {
                    service: service.clone(),
                    log: line,
                },
            );
        }

        match child.stderr.take() {
            Some(stderr) => {
                let reader = BufReader::new(stderr);

                for line in reader.lines().flatten() {
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
            None => (),
        };

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
pub fn running_service(state: tauri::State<AppState>) -> HashMap<String, u32> {
    state.running_service.lock().unwrap().running()
}

#[tauri::command]
pub async fn symlink_env(services: Vec<String>) -> Result<bool, String> {
    for service in services {
        let child = Command::new("ln")
            .args(["-s", "../main.env", "./backend-common/.env"])
            .spawn()
            .unwrap();

        let output = child.wait_with_output();

        if output.is_err() {
            return Err(format!(
                "{}: {}",
                &service,
                output.err().unwrap().to_string()
            ));
        }
    }

    Ok(true)
}

#[tauri::command]
pub async fn clone_repo(
    root_dir: String,
    services: Vec<String>,
    window: Window,
) -> Result<bool, String> {
    let cloned_window = window.clone();

    tokio::spawn(async move {
        for service in services {
            let repo_path = format!("{}/{}", &root_dir.clone(), &service);

            let path = Path::new(repo_path.as_str());

            if path.exists() {
                cloned_window
                    .emit(
                        "service:clone",
                        ServiceClone {
                            service: service.clone(),
                            log: "Already cloned".to_string(),
                        },
                    )
                    .expect("Sending log failed");
            } else {
                let mut child = Command::new("git")
                    .args([
                        "clone",
                        format!("git@github.com:getdokan/{}.git", &service).as_str(),
                    ])
                    .current_dir(&root_dir)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let stdout = child
                    .stdout
                    .take()
                    .expect("Failed to stream output to stdout");

                let reader = BufReader::new(stdout);

                for line in reader.lines().flatten() {
                    cloned_window
                        .emit(
                            "service:clone",
                            ServiceClone {
                                service: service.clone(),
                                log: line,
                            },
                        )
                        .expect("Sending log failed");
                }

                match child.stderr.take() {
                    Some(stderr) => {
                        let reader = BufReader::new(stderr);

                        for line in reader.lines().flatten() {
                            cloned_window
                                .emit(
                                    "service:clone",
                                    ServiceClone {
                                        service: service.clone(),
                                        log: line,
                                    },
                                )
                                .expect("Sending log failed");
                        }
                    }
                    None => (),
                };

                child.wait().expect("Failed to clone the service");
            }
        }
    });

    window
        .emit("service:clone:finish", ())
        .expect("Sending log failed");

    Ok(true)
}
