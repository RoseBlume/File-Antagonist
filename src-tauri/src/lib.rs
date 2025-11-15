mod size;
mod logger;
use std::path::Path;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn find_root() -> String {
    #[cfg(target_os = "windows")]{
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());
        format!("C:\\Users\\{}", username)
    }
    #[cfg(not(target_os = "windows"))]{
        String::from("/")
    }
}

#[tauri::command]
fn get_parent(path: &str) -> String {
    let p = Path::new(path);

    match p.parent() {
        Some(parent) => parent.to_string_lossy().into_owned(),
        None => find_root(), // or return whatever default you prefer
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            find_root,
            get_parent,
            logger::log,
            size::collect_dir_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
