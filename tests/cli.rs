use std::io::{Write};
// Run programs
use std::process::Command;

// Add methods on commands
use assert_cmd::prelude::*;
// Used for writing assertions
use tempfile::NamedTempFile;

const PROGRAM_NAME: &str = "tt";

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;

    cmd.arg("foo").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn args_must_not_be_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    cmd.arg("").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("argument <pattern> must not be empty"));

    Ok(())
}