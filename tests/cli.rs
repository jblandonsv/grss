use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use assert_fs::prelude::*;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("grss")?;
    cmd.arg("main").arg("/Users/jaime/testlabs/grss/src/main.rs");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("fn main() -> Result<()> {"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grss")?;

    cmd.arg("main").arg("file_not_exists");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
