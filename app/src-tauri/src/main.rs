use std::sync::Mutex;
use std::path::PathBuf;
use tauri::State;
use core::scan::{Scanner, ScanStatus, FileInfo};

struct AppState {
    scanner: Mutex<Scanner>,
}

#[tauri::command]
fn scan_start(roots: Vec<String>, state: State<AppState>) {
    let paths: Vec<PathBuf> = roots.into_iter().map(PathBuf::from).collect();
    let mut scanner = state.scanner.lock().unwrap();
    scanner.start(paths);
}

#[tauri::command]
fn scan_status(state: State<AppState>) -> ScanStatus {
    let scanner = state.scanner.lock().unwrap();
    scanner.status()
}

#[tauri::command]
fn scan_results(state: State<AppState>) -> Vec<FileInfo> {
    let scanner = state.scanner.lock().unwrap();
    scanner.results()
}

#[tauri::command]
fn scan_cancel(state: State<AppState>) {
    let mut scanner = state.scanner.lock().unwrap();
    scanner.cancel();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { scanner: Mutex::new(Scanner::new()) })
        .invoke_handler(tauri::generate_handler![scan_start, scan_status, scan_results, scan_cancel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
