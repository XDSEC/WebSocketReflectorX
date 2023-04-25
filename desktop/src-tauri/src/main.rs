// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod connection;

#[tauri::command]
async fn add_ws_connection(addr: String) -> Result<(), String> {
    match connection::add_ws_connection(addr).await {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_alive_connections() -> Result<String, String> {
    match connection::get_alive_ws_connections().await {
        Ok(conns) => Ok(serde_json::to_string(&conns).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_logs() -> String {
    connection::get_logs().await
}

#[tauri::command]
async fn get_dead_connections() -> Result<String, String> {
    match connection::get_dead_ws_connections().await {
        Ok(conns) => Ok(serde_json::to_string(&conns).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn close_connection(id: String) -> Result<(), String> {
    match connection::remove_ws_connection(id).await {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn refresh_latency() -> Result<(), String> {
    match connection::refresh_latency().await {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_ws_connection,
            get_alive_connections,
            get_dead_connections,
            close_connection,
            refresh_latency,
            get_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
