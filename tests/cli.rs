use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn invalid_input_url_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lync")?;

    cmd.arg("-s").arg("www.abc.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("builder error"));

    Ok(())
}

#[test]
fn invalid_input_url_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lync")?;

    cmd.arg("-s").arg("htttps://www.google.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("builder error"));

    Ok(())
}

#[test]
fn invalid_input_host() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lync")?;

    cmd.arg("-s").arg("https://www.abc__.com");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("dns error"));

    Ok(())
}

#[test]
fn invalid_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lync")?;

    cmd.arg("ijk");
    cmd.assert().failure().stderr(predicate::str::contains(
        "For more information try '--help'",
    ));

    Ok(())
}

#[test]
fn no_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lync")?;

    cmd.arg("");
    cmd.assert().failure().stderr(predicate::str::contains(
        "For more information try '--help'",
    ));

    Ok(())
}

#[test]
fn multiple_arguments_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lync")?;

    cmd.arg("-s")
        .arg("https://www.google.com")
        .arg("-d")
        .arg("-m https://www.google.com,https://www.amazon.com")
        .arg("-s")
        .arg("https://www.amazon.com");
    cmd.assert().failure().stderr(predicate::str::contains(
        "For more information try '--help'",
    ));

    Ok(())
}
