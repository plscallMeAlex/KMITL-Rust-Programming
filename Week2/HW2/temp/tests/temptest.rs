use assert_cmd::Command;

#[test]
fn test01(){
    let mut cmd = Command::cargo_bin("temp").unwrap();
    cmd.arg("2");
    cmd.assert().success().stdout("Celsius:-16.666668\n");
}

#[test]
fn test02(){
    let mut cmd = Command::cargo_bin("ctof").unwrap();
    cmd.arg("25");
    cmd.assert().success().stdout("Farenheit:77");
}

// #[test]
// fn test03(){
//     let mut cmd = Command::cargo_bin("ctof").unwrap();
//     cmd.arg("25");
//     cmd.assert().success().stdout("Farenheit:77");
// }