use assert_cmd::Command;
use predicates::prelude::predicate;
use std::io::{BufRead, BufReader};

fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn default_password_length_is_12() {
    let mut cmd = cmd();
    // the colored output add 10 to len of stdout
    let predicate_fn = predicate::function(|x: &str| x.trim().len() == 12 + 10);

    cmd.assert().success().stdout(predicate_fn);
}

#[test]
fn password_arg_length_100() {
    let mut cmd = cmd();
    cmd.args(&["-L", "100"]);
    // the colored output add 10 to len of stdout
    let predicate_fn = predicate::function(|x: &str| x.trim().len() == 100 + 10);

    cmd.assert().success().stdout(predicate_fn);
}

#[test]
fn created_10_passwords() {
    let mut cmd = cmd();
    cmd.args(&["-c", "10"]);

    let assert = cmd.assert().success();

    let output = assert.get_output().stdout.as_slice();
    let reader = BufReader::new(output);

    assert_eq!(reader.lines().count(), 10);
}

#[test]
fn created_111_passwords() {
    let mut cmd = cmd();
    cmd.args(&["-L", "6", "-c", "111"]);

    let assert = cmd.assert().success();

    let output = assert.get_output().stdout.as_slice();
    let reader = BufReader::new(output);

    assert_eq!(reader.lines().count(), 111);
}

#[test]
fn created_50_passwords_save_txt() {
    let mut cmd = cmd();
    cmd.args(&["-L", "6", "-c", "50", "--output", out.txt]);
    // the colored output add 10 to len of stdout
    let predicate_fn = predicate::str::contains("File Saved");

    cmd.assert().success().stdout(predicate_fn);
}
