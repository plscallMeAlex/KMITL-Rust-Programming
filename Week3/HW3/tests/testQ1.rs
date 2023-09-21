use assert_cmd::Command;

#[test]
fn test01(){
    let mut cmd = Command::cargo_bin("q1_grade").unwrap();
    cmd.arg("50").assert().success().stdout("D\n");
}   

#[test]
fn test02(){
    let mut cmd = Command::cargo_bin("q1_grade").unwrap();
    cmd.arg("ssde").assert().success().stdout("Invalid score\n");
} 

#[test]
fn test03(){
    let mut cmd = Command::cargo_bin("q1_grade").unwrap();
    cmd.arg("-2").assert().success().stdout("Invalid score\n");
} 

#[test]
fn test04(){
    let mut cmd = Command::cargo_bin("q1_grade").unwrap();
    cmd.arg("90").assert().success().stdout("A\n");
} 

#[test]
fn test05(){
    let mut cmd = Command::cargo_bin("q1_grade").unwrap();
    cmd.arg("75").assert().success().stdout("B\n");
} 

#[test]
fn test06(){
    let mut cmd = Command::cargo_bin("q1_grade").unwrap();
    cmd.arg("65").assert().success().stdout("C\n");
} 

#[test]
fn test07(){
    let mut cmd = Command::cargo_bin("q1_grade").unwrap();
    cmd.arg("10").assert().success().stdout("Failed with F\n");
} 