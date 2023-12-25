use crate::state::AppState;
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

    tokio::spawn(async move {
        let mut child = Command::new("npx");

        let child = match service.as_str() {
            "storefront" => child.args(["npx", "next", "dev", "-p", "3001"]),
            "dashboard" => child.args(["pnpm", "dev", "--port", "3000"]),
            _ => child
                .args([
                    "ts-node-dev",
                    "--respawn",
                    "--transpile-only",
                    "--exit-child",
                    "-r",
                    "tsconfig-paths/register",
                    "src/index.ts",
                ])
                .env("APP_PORT", port.to_string()),
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

    let p_id = state
        .running_service
        .lock()
        .unwrap()
        .get_pid(&service)
        .unwrap();

    let child = Command::new("kill")
        .args(&["-s", "TERM", p_id.as_str()])
        .spawn();

    if child.is_err() {
        return Err(child.err().unwrap().to_string());
    }

    child.unwrap().wait().expect("Failed to wait");

    send_log(&window, "service:stop", ServiceStop { service });

    Ok("Done".to_string())
}

#[tauri::command]
pub fn running_service(state: tauri::State<AppState>) -> HashMap<String, u32> {
    state.running_service.lock().unwrap().running()
}
