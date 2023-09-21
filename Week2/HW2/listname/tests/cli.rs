use assert_cmd::Command;

#[test]
fn test01(){
    let mut cmd = Command::cargo_bin("listname").unwrap();
    cmd.arg("Kel");
    cmd.arg("New");
    cmd.assert().success().stdout("Player 1: Kel\nPlayer 2: New\n");
}