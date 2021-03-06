//
// Sysinfo
//
// Copyright (c) 2018 Guillaume Gomez
//

extern crate sysinfo;

use sysinfo::ProcessExt;
use sysinfo::SystemExt;

#[test]
fn test_process() {
    let mut s = sysinfo::System::new();
    assert_eq!(s.get_processes().len(), 0);
    s.refresh_processes();
    assert!(s.get_processes().len() != 0);
    #[cfg(not(windows))]
    assert!(s
        .get_processes()
        .values()
        .any(|p| p.exe().to_str().unwrap_or_else(|| "").len() != 0));
}

#[test]
fn test_process_refresh() {
    let mut s = sysinfo::System::new();
    assert_eq!(s.get_processes().len(), 0);
    s.refresh_process(sysinfo::get_current_pid().expect("failed to get current pid"));
    assert_eq!(
        s.get_process(sysinfo::get_current_pid().expect("failed to get current pid"))
            .is_some(),
        true
    );
}

#[test]
#[cfg(windows)]
fn test_get_cmd_line() {
    let p = std::process::Command::new("ping")
        .arg("localhost")
        .arg("-n")
        .arg("2")
        .spawn()
        .unwrap();
    let mut s = sysinfo::System::new();
    s.refresh_processes();
    let process = s.get_process(p.id() as sysinfo::Pid).unwrap();
    assert_eq!(process.cmd(), &["ping", "localhost", "-n", "2"]);
}

#[test]
#[cfg(not(windows))]
fn test_get_cmd_line() {
    let p = std::process::Command::new("timeout")
        .arg("3")
        .arg("ping")
        .arg("localhost")
        .spawn()
        .unwrap();
    let mut s = sysinfo::System::new();
    s.refresh_processes();
    let process = s.get_process(p.id() as sysinfo::Pid).unwrap();
    assert_eq!(process.cmd(), &["timeout", "3", "ping", "localhost"]);
}
