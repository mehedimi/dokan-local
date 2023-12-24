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

#[tauri::command(async)]
pub async fn start_service(
    window: Window,
    state: tauri::State<'_, AppState>,
    root_dir: String,
    port: usize,
    service: String,
) -> Result<u32, String> {
    // npx ts-node-dev --respawn --transpile-only --exit-child -r tsconfig-paths/register src/index.ts

    if state.running_service.lock().unwrap().is_running(&service) {
        return Err(format!("{} is already been running", &service));
    }

    tokio::spawn(async move {
        let mut child = Command::new("npx")
            .args([
                "ts-node-dev",
                "--respawn",
                "--transpile-only",
                "--exit-child",
                "-r",
                "tsconfig-paths/register",
                "src/index.ts",
            ])
            .current_dir(format!("{}/{}", &root_dir, &service))
            .env("APP_PORT", port.to_string())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        window
            .emit_and_trigger(
                "service:start",
                ServiceStart {
                    service: service.clone(),
                    p_id: child.id(),
                },
            )
            .expect("Failed to send start log");

        let stdout = child
            .stdout
            .take()
            .expect("Failed to stream output to stdout");

        let reader = BufReader::new(stdout);

        for line in reader.lines().flatten() {
            window
                .emit(
                    "service:logs",
                    ServiceLog {
                        service: service.clone(),
                        log: line,
                    },
                )
                .expect("Failed to send logs");
        }

        child.wait().expect("Failed to running the service");
    });

    Ok(1)
}

#[tauri::command(async)]
pub async fn stop_service(
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

    Command::new("kill")
        .args(&["-s", "TERM", p_id.as_str()])
        .spawn()
        .expect("Failed to kill the service")
        .wait()
        .expect("Failed to wait");

    window
        .emit_and_trigger("service:stop", ServiceStop { service })
        .expect("Failed to send stop signal");

    Ok("Done".to_string())
}

#[tauri::command]
pub fn running_service(state: tauri::State<AppState>) -> HashMap<String, u32> {
    state.running_service.lock().unwrap().running()
}
