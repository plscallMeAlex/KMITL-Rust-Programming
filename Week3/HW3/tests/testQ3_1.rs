use assert_cmd::Command;

#[test]
fn test01() {
    let mut cmd = Command::cargo_bin("q3_1_rightarrow").unwrap();
    cmd.arg("1").assert().success().stdout("*\n");
}

#[test]
fn test02() {
    let mut cmd = Command::cargo_bin("q3_1_rightarrow").unwrap();
    cmd.assert().success().stdout("\n");
}

#[test]
fn test03() {
    let mut cmd = Command::cargo_bin("q3_1_rightarrow").unwrap();
    cmd.arg("0").assert().success().stdout("\n");
} 

#[test]
fn test04() {
    let mut cmd = Command::cargo_bin("q3_1_rightarrow").unwrap();
    cmd.arg("3").assert().success().stdout("*\n**\n***\n**\n*\n");
}

#[test]
fn test05() {
    let mut cmd = Command::cargo_bin("q3_1_rightarrow").unwrap();
    cmd.arg("sdsd").assert().success().stdout("\n");
}