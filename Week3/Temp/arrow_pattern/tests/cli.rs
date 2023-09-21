use assert_cmd::Command;
use std::fs;

// type Result = Result<(), Box<dyn std::error::Error>>;
// type Outcome = Result<(), Box<dyn std::error::Error>>;
// #[test]
fn test01() {
    let mut cmd = Command::cargo_bin("arrow_right").unwrap();
    cmd.arg("1").assert().success().stdout("*\n");
}

#[test]
fn test02() {
    let mut cmd = Command::cargo_bin("arrow_right").unwrap();
    cmd.assert().success().stdout("\n");
}

#[test]
fn test03() {
    let mut cmd = Command::cargo_bin("arrow_right").unwrap();
    cmd.arg("0").assert().success().stdout("Nothing Happened\n");
} 

#[test]
fn test04() {
    let mut cmd = Command::cargo_bin("arrow_right").unwrap();
    cmd.arg("3").assert().success().stdout("*\n**\n***\n**\n*\n");
}
// #[test]
// fn test05(){
//     run(&"5", "tests/5_arrow.txt").expect("test failed");
// }

// fn run(arg: &str, file: &str) -> Testresult {
//     let expected: Vec<u8> = fs::read(file)?;
//     Command::cargo_bin("arrow_right")?.arg(arg).assert().success().stdout(expected);
//     assert_eq!(stdout, expected, "Test failed");
//     Ok(())
// }