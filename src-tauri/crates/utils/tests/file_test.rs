use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use serde_json::json;
use std::sync::mpsc::{self, Sender};
use std::collections::VecDeque;

const OUTPUT_PATH: &str = "tests/outputs/output.json";

/// Worker thread: receives paths, scans them, sends discovered items + new dirs back.
fn worker(rx: Arc<Mutex<std::sync::mpsc::Receiver<PathBuf>>>, tx: Sender<(Option<(PathBuf, u64, String)>, Vec<PathBuf>)>, current: Arc<Mutex<String>>) {
    while let Ok(path) = rx.lock().unwrap().recv() {
        if let Ok(mut lock) = current.lock() {
            *lock = path.display().to_string();
        }

        let meta = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(_) => { let _ = tx.send((None, vec![])); continue; }
        };

        if meta.file_type().is_symlink() {
            let _ = tx.send((None, vec![]));
            continue;
        }

        // Skip files entirely
        else {
            let _ = tx.send((None, vec![]));
            continue;
        }

        if meta.is_dir() {
            let mut children = vec![];
            let mut total = 0;

            let rd = match fs::read_dir(&path) {
                Ok(r) => r,
                Err(e) => {
                    if e.kind() == io::ErrorKind::PermissionDenied {
                        let _ = tx.send((Some((path.clone(), 0, "Dir".into())), vec![]));
                        continue;
                    }
                    let _ = tx.send((None, vec![]));
                    continue;
                }
            };

            for entry in rd {
                if let Ok(e) = entry {
                    children.push(e.path());
                }
            }

            // Directory size will be filled later as children are processed, so store 0.
            let _ = tx.send((Some((path.clone(), 0, "Dir".into())), children));
        }
    }
}

#[test]
fn scan_entire_disk_to_json() -> io::Result<()> {
    #[cfg(target_os = "windows")]
    let root = PathBuf::from("C:/");
    #[cfg(not(target_os = "windows"))]
    let root = PathBuf::from("/");

    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&OUTPUT_PATH)?;
    file.write_all(b"[")?;

    let current_path = Arc::new(Mutex::new(String::new()));
    let progress_path = Arc::clone(&current_path);

    thread::spawn(move || {
        let mut last = Instant::now();
        loop {
            if last.elapsed() >= Duration::from_secs(10) {
                if let Ok(lock) = progress_path.lock() {
                    eprintln!("Scanning: {}", *lock);
                }
                last = Instant::now();
            }
            thread::sleep(Duration::from_millis(200));
        }
    });

    let (path_tx, raw_rx) = mpsc::channel::<PathBuf>();
    let path_rx = Arc::new(Mutex::new(raw_rx));
    let (res_tx, res_rx) = mpsc::channel::<(Option<(PathBuf, u64, String)>, Vec<PathBuf>)>();

    // Spawn worker threads
    let threads = 8;
    for _ in 0..threads {
        let rx = Arc::clone(&path_rx);
        let rx = Arc::clone(&path_rx);
        let tx = res_tx.clone();
        let cp = Arc::clone(&current_path);
        thread::spawn(move || worker(rx, tx, cp));
    }

    path_tx.send(root.clone()).unwrap();

    let mut pending = 1usize;
    let mut first = true;

    while pending > 0 {
        if let Ok((item_opt, new_dirs)) = res_rx.recv() {
            pending -= 1;

            for d in new_dirs.into_iter() {
                pending += 1;
                let _ = path_tx.send(d);
            }

            if let Some((path, size, kind)) = item_opt {
                if kind == "Dir" {
                    let val = json!({"path": path.display().to_string(), "size": size, "kind": kind});
                    if !first {
                        file.write_all(b",")?;
                    }
                    first = false;
                    file.write_all(serde_json::to_string(&val).unwrap().as_bytes())?;
                    file.flush()?;
                }
            }
        
        }
    }

    file.write_all(b"]")?;
    file.flush()?;

    Ok(())
}

