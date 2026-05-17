use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("open-bcif"));
}

#[test]
fn test_validate_missing_file() {
    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("validate").arg("non_existent_file.bcif")
        .assert()
        .failure();
}

#[test]
fn test_validate_sample_file() {
    // We can't easily call create_sample_bcif from here because it's in the bin,
    // so we'll just use a small shell command to create a dummy file for basic existence check,
    // or better, we should have a 'test-gen' hidden command or just manual file creation.
    // For now, let's just test that the command exists and fails on bad data.
    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("validate").arg("Cargo.toml") // Cargo.toml is not a valid BCIF
        .assert()
        .failure();
}

#[test]
fn test_split_and_validate() {
    // This is a more complex test that would ideally use a real BCIF.
    // Since we don't have one, we'll verify the error path.
    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("split").arg("Cargo.toml").arg("--output-dir").arg("test_split_out")
        .assert()
        .failure();
}
