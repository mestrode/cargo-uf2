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
fn test_verify_valid_uf2() {
    let mut cmd = cargo_uf2();
    cmd.args(["verify", "tests/fixtures/uf2/valid.uf2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ UF2 file is valid"))
        .stdout(predicate::str::contains("Blocks: 1"));
}

#[test]
fn test_verify_empty_uf2() {
    let temp_dir = tempdir().unwrap();
    let empty_uf2 = temp_dir.path().join("empty.uf2");
    fs::write(
        &empty_uf2,
        include_bytes!("../tests/fixtures/uf2/empty.uf2"),
    )
    .unwrap();

    let mut cmd = cargo_uf2();
    cmd.args(["verify", empty_uf2.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ UF2 file is valid"));
}

#[test]
fn test_verify_invalid_magic() {
    let mut cmd = cargo_uf2();
    cmd.args(["verify", "tests/fixtures/uf2/invalid_magic.uf2"])
        .assert()
        .failure();
}
