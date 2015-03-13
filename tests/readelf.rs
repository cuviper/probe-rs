#![feature(asm)]

#[macro_use] #[no_link]
extern crate probe;

use std::env;
use std::process::Command;

#[test]
fn check_notes() {
    // First let's create probes with and without arguments
    probe!(test, foo);
    probe!(test, bar, 42);

    // Now make sure readelf can find "stapsdt" ELF notes in this test executable
    let test_exe = env::current_exe().unwrap();
    let output = Command::new("readelf").arg("-n").arg(&test_exe).output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let count = String::from_utf8_lossy(&output.stdout).lines()
        .filter(|line| line.contains("stapsdt")).count();
    assert_eq!(count, 2);
}
