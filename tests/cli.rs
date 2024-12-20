use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure();  // 引数がないので
}

#[test]
fn runs2() {
    Command::cargo_bin("echor")
    .unwrap()
    .arg("hello")  // 単一の引数
    .assert()
    .success();
}

#[test]
fn runs3() {
    Command::cargo_bin("echor")
    .unwrap()
    .args(&["hello", "world"])  // 複数の引数
    .assert()
    .success();
}

// 出力を確認するには
#[test]
fn runs4() {
    Command::cargo_bin("echor")
    .unwrap()
    .args(&["hello", "world"])
    .assert()
    .stdout(predicate::str::contains("hello world"));
}

#[test]
fn runs4b() {
    let expected = format!("{} {}\n", "hello", "world");
    Command::cargo_bin("echor")
        .unwrap()
        .args(&["hello", "world"])
        .assert()
        .stdout(predicate::eq(expected));
}