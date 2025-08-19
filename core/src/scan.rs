use std::path::{PathBuf, Path};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use rayon::prelude::*;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub modified: i64,
    pub accessed: i64,
    pub created: i64,
    pub ext: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScanStatus {
    pub running: bool,
    pub scanned_files: u64,
    pub scanned_bytes: u64,
    pub current_path: Option<String>,
}

impl Default for ScanStatus {
    fn default() -> Self {
        Self { running: false, scanned_files: 0, scanned_bytes: 0, current_path: None }
    }
}

#[derive(Clone)]
pub struct PathFilter {
    blocked: Vec<PathBuf>,
}

impl Default for PathFilter {
    fn default() -> Self {
        let mut blocked = Vec::new();
        if let Ok(p) = std::env::var("WINDIR") { blocked.push(PathBuf::from(p)); }
        if let Ok(p) = std::env::var("ProgramFiles") { blocked.push(PathBuf::from(p)); }
        if let Ok(p) = std::env::var("ProgramFiles(x86)") { blocked.push(PathBuf::from(p)); }
        Self { blocked }
    }
}

impl PathFilter {
    pub fn should_skip(&self, p: &Path) -> bool {
        self.blocked.iter().any(|b| p.starts_with(b))
    }
}

pub struct Scanner {
    running: Arc<AtomicBool>,
    cancel: Arc<AtomicBool>,
    status: Arc<Mutex<ScanStatus>>,
    results: Arc<Mutex<Vec<FileInfo>>>,
    roots: Vec<PathBuf>,
    filter: PathFilter,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            cancel: Arc::new(AtomicBool::new(false)),
            status: Arc::new(Mutex::new(ScanStatus::default())),
            results: Arc::new(Mutex::new(Vec::new())),
            roots: Vec::new(),
            filter: PathFilter::default(),
        }
    }

    pub fn start(&mut self, roots: Vec<PathBuf>) {
        if self.running.load(Ordering::SeqCst) { return; }
        self.roots = roots.clone();
        self.cancel.store(false, Ordering::SeqCst);
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let cancel = self.cancel.clone();
        let status = self.status.clone();
        let results = self.results.clone();
        let filter = self.filter.clone();
        let allow = Arc::new(roots);
        {
            let mut st = status.lock().unwrap();
            st.running = true;
        }
        std::thread::spawn(move || {
            results.lock().unwrap().clear();
            for root in allow.iter() {
                if cancel.load(Ordering::SeqCst) { break; }
                let iter = WalkDir::new(root).follow_links(false).into_iter();
                let allow_inner = allow.clone();
                let cancel_inner = cancel.clone();
                let results_inner = results.clone();
                let status_inner = status.clone();
                let filter_inner = filter.clone();
                iter.filter_map(|e| e.ok()).par_bridge().for_each(move |entry| {
                    if cancel_inner.load(Ordering::SeqCst) { return; }
                    let path = entry.path().to_path_buf();
                    if filter_inner.should_skip(&path) && !allow_inner.iter().any(|r| path.starts_with(r)) {
                        return;
                    }
                    if entry.file_type().is_file() {
                        let meta = match entry.metadata() { Ok(m) => m, Err(_) => return };
                        let size = meta.len();
                        let modified = meta.modified().ok().and_then(to_secs).unwrap_or(0);
                        let accessed = meta.accessed().ok().and_then(to_secs).unwrap_or(0);
                        let created = meta.created().ok().and_then(to_secs).unwrap_or(0);
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                        {
                            let mut res = results_inner.lock().unwrap();
                            res.push(FileInfo {
                                path: path.to_string_lossy().to_string(),
                                size,
                                modified,
                                accessed,
                                created,
                                ext: ext.to_string(),
                            });
                        }
                        {
                            let mut st = status_inner.lock().unwrap();
                            st.scanned_files += 1;
                            st.scanned_bytes += size;
                            st.current_path = Some(path.to_string_lossy().to_string());
                        }
                    }
                });
            }
            running.store(false, Ordering::SeqCst);
            let mut st = status.lock().unwrap();
            st.running = false;
            st.current_path = None;
        });
    }

    pub fn status(&self) -> ScanStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn results(&self) -> Vec<FileInfo> {
        self.results.lock().unwrap().clone()
    }

    pub fn cancel(&mut self) {
        self.cancel.store(true, Ordering::SeqCst);
    }
}

fn to_secs(time: SystemTime) -> Option<i64> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs() as i64)
}

use std::collections::HashMap;

pub fn summarize_by_folder(files: &[FileInfo]) -> Vec<(PathBuf, u64)> {
    let mut map: HashMap<PathBuf, u64> = HashMap::new();
    for f in files {
        let p = Path::new(&f.path);
        if let Some(parent) = p.parent() {
            *map.entry(parent.to_path_buf()).or_insert(0) += f.size;
        }
    }
    let mut v: Vec<(PathBuf, u64)> = map.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    v
}
