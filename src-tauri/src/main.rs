// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod cosmos_api;

#[tauri::command]
fn send_coins(params: serde_json::Value) -> Result<String, String> {
    let amount: &str = params["amount"].as_str().ok_or_else(|| "Missing or invalid 'amount'")?;
    let recipient: &str = params["recipient"].as_str().ok_or_else(|| "Missing or invalid 'recipient'")?;
    match cosmos_api::send_coins(amount.to_string(), recipient.to_string()) {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn query_balance(address: String) -> Result<String, String> {
    match cosmos_api::query_balance(&address) {
        Ok(balance_info) => Ok(balance_info),
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_coins, query_balance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}