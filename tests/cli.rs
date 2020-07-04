use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::{self, Write};
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doent/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
fn too_less_argment() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));

    cmd.arg("test");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));

    Ok(())
}
