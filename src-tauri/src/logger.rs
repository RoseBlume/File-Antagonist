#[tauri::command]
pub fn log(message: &str) {
    println!("{}", message);
}