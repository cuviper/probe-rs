#![feature(asm)]

use probe::probe;
use std::env;
use std::process::Command;

#[test]
fn check_notes() {
    // First let's create probes with and without arguments
    probe!(test, foo);
    probe!(test, bar, 42);

    // Now make sure readelf can find "stapsdt" ELF notes in this test executable
    let test_exe = env::current_exe().unwrap();
    let output = Command::new("readelf")
        .arg("-n")
        .arg(&test_exe)
        .output()
        .unwrap();
    assert!(output.status.success());

    for error in String::from_utf8_lossy(&output.stderr).lines() {
        if error.contains("Warning: Gap in build notes detected") {
            continue;
        }
        panic!("{}", error);
    }

    let count = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.contains("NT_STAPSDT"))
        .count();
    assert_eq!(count, 2);
}
