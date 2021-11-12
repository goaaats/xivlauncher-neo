#![cfg(test)]

use super::*;

// TODO: Make all of this platform-agnostic

#[test]
fn run_command_does_not_panic() {
    run_command("cmd.exe", &[] as &[&str]);
}

#[test]
fn is_process_running_returns_true_when_running() {
    // This should be running during the test, at least in normal test environments
    assert!(is_process_running("cargo.exe"));
}

#[test]
fn is_process_running_returns_false_when_not_running() {
    assert!(!is_process_running("^.exe"));
}