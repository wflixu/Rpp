use assert_cmd::Command;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rppm").unwrap();
    cmd.assert().success().stdout("hello, world!!!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
