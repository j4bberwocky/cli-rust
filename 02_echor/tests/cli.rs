use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello girls"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "girls"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["-n", "Hello girls"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "girls"], "tests/expected/hello2.n.txt")
}

// Utility function
fn run(args: &[&str], expected_file: &str) -> TestResult {
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(fs::read_to_string(expected_file)?);
    Ok(())
}
