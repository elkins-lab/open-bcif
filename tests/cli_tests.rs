use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("open-bcif"));
}

#[test]
fn test_validate_success() {
    let path = "test_valid.bcif";
    open_bcif::test_utils::create_sample_bcif(path).unwrap();

    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("validate").arg(path).assert().success();

    fs::remove_file(path).unwrap();
}

#[test]
fn test_split_success() {
    let path = "test_split.bcif";
    let out_dir = "test_split_out";
    open_bcif::test_utils::create_sample_bcif(path).unwrap();
    let _ = fs::remove_dir_all(out_dir);

    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("split")
        .arg(path)
        .arg("--output-dir")
        .arg(out_dir)
        .assert()
        .success();

    // Verify files were created
    let entries: Vec<_> = fs::read_dir(out_dir).unwrap().collect();
    assert!(entries.len() >= 1);

    fs::remove_file(path).unwrap();
    fs::remove_dir_all(out_dir).unwrap();
}

#[test]
fn test_merge_success() {
    let in1 = "test_m1.bcif";
    let in2 = "test_m2.bcif";
    let out = "test_merged_cli.bcif";
    open_bcif::test_utils::create_sample_bcif(in1).unwrap();
    open_bcif::test_utils::create_sample_bcif(in2).unwrap();

    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("merge")
        .arg(in1)
        .arg(in2)
        .arg("--output")
        .arg(out)
        .assert()
        .success();

    assert!(fs::metadata(out).is_ok());

    fs::remove_file(in1).unwrap();
    fs::remove_file(in2).unwrap();
    fs::remove_file(out).unwrap();
}

#[test]
fn test_convert_success() {
    let path = "test_conv.bcif";
    let out = "test_conv.cif";
    open_bcif::test_utils::create_sample_bcif(path).unwrap();

    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("convert")
        .arg(path)
        .arg("--output")
        .arg(out)
        .arg("--format")
        .arg("cif")
        .assert()
        .success();

    let content = fs::read_to_string(out).unwrap();
    assert!(content.contains("data_TEST_BLOCK_1"));

    fs::remove_file(path).unwrap();
    fs::remove_file(out).unwrap();
}

#[test]
fn test_validate_missing_file() {
    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("validate")
        .arg("non_existent_file.bcif")
        .assert()
        .failure();
}

#[test]
fn test_validate_corrupted_file() {
    let path = "test_corrupt_cli.bcif";
    fs::write(path, vec![0x93, 0x01, 0x02, 0x03]).unwrap(); // Random bytes

    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("validate").arg(path).assert().failure();

    fs::remove_file(path).unwrap();
}

#[test]
fn test_convert_invalid_format() {
    let path = "test_fmt.bcif";
    open_bcif::test_utils::create_sample_bcif(path).unwrap();

    let mut cmd = Command::cargo_bin("open-bcif").unwrap();
    cmd.arg("convert")
        .arg(path)
        .arg("--output")
        .arg("out.txt")
        .arg("--format")
        .arg("yaml") // Unsupported
        .assert()
        .failure();

    fs::remove_file(path).unwrap();
}
