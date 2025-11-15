use serde_json::Value;
use std::thread;
use std::path::Path;
use utils::collect_dirs;
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FinishedSearching {
  obj: Value,
}


#[tauri::command]
pub fn collect_dir_info(app: AppHandle, perfect_path: String) {
    println!("Updating Data");
    thread::spawn(move || {
        let (_, obj) = collect_dirs(Path::new(&perfect_path.clone()), true);
        println!("Finished Searching");
        app.emit("finished-searching", FinishedSearching { obj }).unwrap();
    });
    println!("Finished main function");
}





