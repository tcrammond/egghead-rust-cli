use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use color_eyre::eyre::Result;

#[test]
/// make sure help runs. This indicates the binary works
fn test_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
/// make sure we have a write command by running `garden write --help`
fn test_write_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("write").arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
fn test_write_with_title_argument() {
    let fake_editor_script = std::env::current_dir()
        .expect("Expected to be in a dir")
        .join("tests")
        .join("fake-editor.sh");

    if !fake_editor_script.exists() {
        panic!("Could ont find the fake editor script");
    }

    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("garden").unwrap();

    let assert = cmd
        .env("EDITOR", fake_editor_script.as_os_str())
        .env("GARDEN_PATH", temp_dir.path())
        .arg("write")
        .arg("-t")
        .arg("test title")
        .write_stdin("Y\n".as_bytes())
        .assert();

    assert.success();

    temp_dir.child("test-title.md").assert(predicate::path::exists());
}

#[test]
fn test_write_with_implicit_title() {
    let fake_editor_script = std::env::current_dir()
        .expect("Expected to be in a dir")
        .join("tests")
        .join("fake-editor.sh");

    if !fake_editor_script.exists() {
        panic!("Could ont find the fake editor script");
    }

    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("garden").unwrap();

    let assert = cmd
        .env("EDITOR", fake_editor_script.as_os_str())
        .env("GARDEN_PATH", temp_dir.path())
        .arg("write")
        .write_stdin("Y\n".as_bytes())
        .assert();

    assert.success();

    temp_dir.child("testing.md").assert(predicate::path::exists());
}
