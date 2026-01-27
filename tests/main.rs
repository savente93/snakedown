use color_eyre::eyre::Result;
use tempfile::tempdir;

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;

use std::process::Command;

#[test]
fn test_cli_with_all_options() -> Result<()> {
    let tempdir = tempdir()?;

    let mut cmd = cargo_bin_cmd!();
    cmd.arg("-p")
        .arg("tests/test_pkg")
        .arg("-s")
        .arg(tempdir.path())
        .arg("--skip-undoc")
        .arg("--skip-private")
        .arg("-e")
        .arg("test_pkg/miss_spelled_ref.py")
        .arg("-e")
        .arg("test_pkg/excluded_file.py")
        .arg("--exclude")
        .arg("test_pkg/excluded_module")
        .arg("-vv");
    let assertion = cmd.assert();

    assertion.success();

    Ok(())
}
#[test]
fn test_cli_with_zola() -> Result<()> {
    let tempdir = tempdir()?;

    let target_dir = tempdir.path().join("zola_test_site");

    // I'm too lazy to implement copying the file tree in rust
    let _ = Command::new("cp")
        .arg("-r")
        .arg("tests/zola_test_site/")
        .arg(&target_dir)
        .assert();

    let mut cmd = cargo_bin_cmd!();
    cmd.arg("-p")
        .arg("tests/test_pkg")
        .arg("-s")
        .arg(&target_dir)
        .arg("--skip-undoc")
        .arg("--skip-private")
        .arg("-e")
        .arg("test_pkg/miss_spelled_ref.py")
        .arg("-e")
        .arg("test_pkg/excluded_file.py")
        .arg("--exclude")
        .arg("test_pkg/excluded_module")
        .arg("--ssg")
        .arg("zola")
        .arg("-vv");
    let snakedown_assertion = cmd.assert();

    snakedown_assertion.success();

    let zola_cmd_assert = Command::new("zola")
        .current_dir(&target_dir)
        .arg("build")
        .assert();

    zola_cmd_assert.success();

    Ok(())
}
