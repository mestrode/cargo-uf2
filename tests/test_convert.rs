use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

const CARGO_UF2: &str = "cargo-uf2";

fn cargo_uf2() -> Command {
    let cmd = Command::cargo_bin(CARGO_UF2).unwrap();
    cmd
}

#[test]
fn test_convert_binary_to_uf2() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("input.bin");
    let output_path = temp_dir.path().join("output.uf2");

    fs::write(&input_path, b"test data for conversion").unwrap();

    let mut cmd = cargo_uf2();
    cmd.args([
        "convert",
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Converting"))
    .stdout(predicate::str::contains("Written"));

    assert!(output_path.exists());
    let output_content = fs::read(&output_path).unwrap();
    assert!(!output_content.is_empty());
}

#[test]
fn test_convert_uf2_to_binary() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("input.uf2");
    let output_path = temp_dir.path().join("output.bin");

    fs::write(
        &input_path,
        include_bytes!("../tests/fixtures/uf2/valid.uf2"),
    )
    .unwrap();

    let mut cmd = cargo_uf2();
    cmd.args([
        "convert",
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Converting"))
    .stdout(predicate::str::contains("Read"));

    assert!(output_path.exists());
    let output_content = fs::read(&output_path).unwrap();
    assert!(!output_content.is_empty());
}

#[test]
fn test_convert_with_base_address() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("input.bin");
    let output_path = temp_dir.path().join("output.uf2");

    fs::write(&input_path, b"test data").unwrap();

    let mut cmd = cargo_uf2();
    cmd.args([
        "convert",
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        "-b",
        "0x08000000",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Converting"));

    assert!(output_path.exists());
}

#[test]
fn test_convert_with_family_id() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("input.bin");
    let output_path = temp_dir.path().join("output.uf2");

    fs::write(&input_path, b"test data").unwrap();

    let mut cmd = cargo_uf2();
    cmd.args([
        "convert",
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        "-f",
        "0xADA52840",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Family ID"));

    assert!(output_path.exists());
}

#[test]
fn test_convert_with_page_size() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("input.bin");
    let output_path = temp_dir.path().join("output.uf2");

    fs::write(&input_path, b"test data").unwrap();

    let mut cmd = cargo_uf2();
    cmd.args([
        "convert",
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        "-p",
        "256",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Page Size: 256 (user-provided)"));

    assert!(output_path.exists());
}

#[test]
fn test_convert_auto_output_extension() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("input.bin");
    let expected_output = temp_dir.path().join("input.uf2");

    fs::write(&input_path, b"test data").unwrap();

    let mut cmd = cargo_uf2();
    cmd.args(["convert", input_path.to_str().unwrap()])
        .assert()
        .success();

    assert!(expected_output.exists());
}

#[test]
fn test_convert_uf2_to_bin_auto_extension() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("input.uf2");
    let expected_output = temp_dir.path().join("input.bin");

    fs::write(
        &input_path,
        include_bytes!("../tests/fixtures/uf2/valid.uf2"),
    )
    .unwrap();

    let mut cmd = cargo_uf2();
    cmd.args(["convert", input_path.to_str().unwrap()])
        .assert()
        .success();

    assert!(expected_output.exists());
}
