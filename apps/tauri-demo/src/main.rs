#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn ping() -> &'static str {
    "pong"
}

fn main() {
    if let Err(error) = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!("tauri.conf.json"))
    {
        eprintln!("tauri application failed to run: {error}");
        std::process::exit(1);
    }
}
