use core::scan::Scanner;
use std::fs;
use std::time::Duration;

#[test]
fn scan_collects_files() {
    let dir = tempfile::tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = dir.path().join("b.txt");
    fs::write(&file1, b"hi").unwrap();
    fs::write(&file2, b"there").unwrap();

    let mut scanner = Scanner::new();
    scanner.start(vec![dir.path().to_path_buf()]);
    loop {
        if !scanner.status().running { break; }
        std::thread::sleep(Duration::from_millis(10));
    }
    let results = scanner.results();
    assert!(results.len() >= 2);
}
