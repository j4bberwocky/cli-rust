use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";
const TABS: &str = "tests/inputs/tabs.txt";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
fn run_stdin(
    input_file: &str,
    args: &[&str],
    expected_file: &str,
) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// -------------------------------------------------- FAIL
// cargo run -- tests/inputs/the-bustle.txt &> tmp.txt
// cargo build
// .\target\debug\catr.exe tests/inputs/the-bustle.txt > tmpCatr.txt
// cat tests/inputs/the-bustle.txt > tmpCat.txt
#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], "tests/expected_windows/the-bustle.txt.stdin.out")
}

// --------------------------------------------------
#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected_windows/the-bustle.txt.n.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected_windows/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b_e() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-bE", "-"],
        "tests/expected_windows/the-bustle.txt.b.E.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected_windows/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected_windows/empty.txt.n.out")
}

// --------------------------------------------------
#[test]
fn empty_b() -> TestResult {
    run(&["-b", EMPTY], "tests/expected_windows/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected_windows/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected_windows/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> TestResult {
    run(&["-b", FOX], "tests/expected_windows/fox.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox_e() -> TestResult {
    run(&["-E", FOX], "tests/expected_windows/fox.txt.E.out")
}

// --------------------------------------------------
#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected_windows/spiders.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> TestResult {
    run(&["--number", SPIDERS], "tests/expected_windows/spiders.txt.n.out")
}

// --------------------------------------------------
#[test]
fn spiders_b() -> TestResult {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected_windows/spiders.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn spiders_e() -> TestResult {
    run(
        &["--show-ends", SPIDERS],
        "tests/expected_windows/spiders.txt.E.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected_windows/the-bustle.txt.out")
}

// --------------------------------------------------
#[test]
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected_windows/the-bustle.txt.n.out")
}

// --------------------------------------------------
#[test]
fn bustle_e() -> TestResult {
    run(&["-E", BUSTLE], "tests/expected_windows/the-bustle.txt.E.out")
}

// --------------------------------------------------
#[test]
fn bustle_n_e() -> TestResult {
    run(&["-nE", BUSTLE], "tests/expected_windows/the-bustle.txt.n.E.out")
}

// --------------------------------------------------
#[test]
fn bustle_b() -> TestResult {
    run(&["-b", BUSTLE], "tests/expected_windows/the-bustle.txt.b.out")
}

// --------------------------------------------------
#[test]
fn all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected_windows/all.out")
}

// --------------------------------------------------
#[test]
fn all_n() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected_windows/all.n.out")
}

// --------------------------------------------------
#[test]
fn all_b() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected_windows/all.b.out")
}

// --------------------------------------------------
#[test]
fn tab_stdin_t() -> TestResult {
    run_stdin(
        TABS,
        &["-T", "-"],
        "tests/expected_windows/tabs.txt.T.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn tab_stdin_t_b() -> TestResult {
    run_stdin(
        TABS,
        &["-Tb", "-"],
        "tests/expected_windows/tabs.txt.T.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn tab_stdin_n_t() -> TestResult {
    run_stdin(
        TABS,
        &["-nT", "-"],
        "tests/expected_windows/tabs.txt.n.T.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn tab_stdin_n_e() -> TestResult {
    run_stdin(
        TABS,
        &["-nE", "-"],
        "tests/expected_windows/tabs.txt.n.E.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn tab_stdin_t_b_e() -> TestResult {
    run_stdin(
        TABS,
        &["-TbE", "-"],
        "tests/expected_windows/tabs.txt.T.b.E.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn tab_stdin_e_n_t() -> TestResult {
    run_stdin(
        TABS,
        &["-EnT", "-"],
        "tests/expected_windows/tabs.txt.E.n.T.stdin.out",
    )
}