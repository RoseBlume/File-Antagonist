use serde_json::{Map, Value};
use std::fs;
use std::path::{Path, PathBuf};

#[inline]
fn format_size(size: u64) -> String {
    const UNITS: [&str; 9] = ["B","KB","MB","GB","TB","PB","EB","ZB","YB"];
    let mut s = size as f64;
    let mut i = 0;

    while s >= 1024.0 && i < UNITS.len() - 1 {
        s /= 1024.0;
        i += 1;
    }

    if i == 0 {
        format!("{} {}", size, UNITS[i])
    } else {
        let mut out = String::with_capacity(12);
        use std::fmt::Write;
        write!(out, "{:.2} {}", s, UNITS[i]).unwrap();
        out
    }
}

#[inline]
fn extract_name(path: &Path) -> String {
    match path.file_name() {
        Some(s) => s.to_string_lossy().into_owned(),
        None => path.to_string_lossy().into_owned(),
    }
}

pub fn collect_dirs(path: &Path, file_mode: bool) -> (u64, Value) {
    let own_meta = fs::metadata(path).ok();
    let own_size = own_meta.map(|m| m.len()).unwrap_or(0);
    let name = extract_name(path);

    let entries_iter = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => {
            let mut obj = Map::new();
            obj.insert("path".into(), Value::String(path.to_string_lossy().into_owned()));
            obj.insert("name".into(), Value::String(name));
            obj.insert("size".into(), Value::String(format_size(own_size)));
            obj.insert("bitsize".into(), Value::String(own_size.to_string()));
            obj.insert("subdirs".into(), Value::Array(Vec::new()));
            obj.insert("type".into(), Value::String("directory".into()));
            obj.insert("permission_denied".into(), Value::Bool(true));
            return (own_size, Value::Object(obj));
        }
    };

    let mut total_size = 0u64;

    // Pre-allocate for speed
    let mut dir_items: Vec<(u64, Value)> = Vec::with_capacity(64);
    let mut file_items: Vec<(u64, Value)> = Vec::with_capacity(256);

    for entry in entries_iter {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let p: PathBuf = entry.path();
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.is_dir() {
            let (sub_size, sub_json) = collect_dirs(&p, false);
            total_size += sub_size;
            dir_items.push((sub_size, sub_json));
        } else if meta.is_file() {
            let file_size = meta.len();
            total_size += file_size;

            if file_mode {
                let mut obj = Map::new();
                obj.insert("path".into(), Value::String(p.to_string_lossy().into_owned()));
                obj.insert("name".into(), Value::String(extract_name(&p)));
                obj.insert("size".into(), Value::String(format_size(file_size)));
                obj.insert("bitsize".into(), Value::String(file_size.to_string()));
                obj.insert("subdirs".into(), Value::Array(Vec::new()));
                obj.insert("type".into(), Value::String("file".into()));

                file_items.push((file_size, Value::Object(obj)));
            }
        }
    }

    let mut all = Vec::with_capacity(dir_items.len() + file_items.len());
    all.extend(dir_items);
    all.extend(file_items);

    let mut subdir_json = Vec::with_capacity(all.len());

    for (size, mut obj) in all {
        let percent = if total_size > 0 {
            (size as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        if let Some(map) = obj.as_object_mut() {
            map.insert("percent".into(), Value::String(format!("{:.2}%", percent)));
            subdir_json.push(Value::Object(map.clone()));
        }
    }

    if total_size == 0 {
        total_size = own_size;
    }

    let mut root = Map::new();
    root.insert("path".into(), Value::String(path.to_string_lossy().into_owned()));
    root.insert("name".into(), Value::String(name));
    root.insert("size".into(), Value::String(format_size(total_size)));
    root.insert("bitsize".into(), Value::String(total_size.to_string()));
    root.insert("subdirs".into(), Value::Array(subdir_json));
    root.insert("type".into(), Value::String("directory".into()));

    (total_size, Value::Object(root))
}
