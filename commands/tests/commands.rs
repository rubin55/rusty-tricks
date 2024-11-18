use assert_cmd::Command;

#[test]
fn hello_output_equals() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_exits_zero() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_exits_one() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
