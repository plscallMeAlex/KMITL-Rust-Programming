use assert_cmd::Command;

#[test]
fn test01(){
    let mut cmd = Command::cargo_bin("calculate_slope").unwrap();
    cmd.arg("1");
    cmd.arg("2");
    cmd.arg("3");
    cmd.arg("4");
    cmd.assert().success();
}

#[test]
fn test02(){
    let mut cmd = Command::cargo_bin("calculate_slope").unwrap();
    cmd.arg("4");
    cmd.arg("-1");
    cmd.arg("1");
    cmd.arg("2");
    cmd.assert().success();
}

#[test]
fn test03(){
    let mut cmd = Command::cargo_bin("calculate_slope").unwrap();
    cmd.arg("w");
    cmd.arg("w");
    cmd.arg("w");
    cmd.arg("w");
    cmd.assert().success();
}