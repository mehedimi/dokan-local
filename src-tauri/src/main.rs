// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::service::{ServiceStart, ServiceStop};
use crate::state::{AppState};
use std::sync::{Arc};
use tauri::{Manager};

mod commands;
mod state;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let state = app.state::<AppState>().clone();
            let main_window = app.get_window("main").unwrap();

            let state_clone = Arc::clone(&state.running_service);

            let state_clone_2 = Arc::clone(&state.running_service);

            main_window.listen("service:start", move |event| {
                let payload: ServiceStart =
                    serde_json::from_str(event.payload().unwrap()).expect("Failed to parse JSON");
                state_clone
                    .lock()
                    .unwrap()
                    .add(payload.service, payload.p_id);
            });

            main_window.listen("service:stop", move |event| {
                let payload: ServiceStop =
                    serde_json::from_str(event.payload().unwrap()).expect("Failed to parse JSON");
                state_clone_2.lock().unwrap().remove(&payload.service);
            });

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::service::start_service,
            commands::service::stop_service,
            commands::service::running_service
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
