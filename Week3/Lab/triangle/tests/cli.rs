use assert_cmd::Command;

#[test]
fn test01(){
   let mut cmd = Command::cargo_bin("triangle").unwrap();
   cmd.arg("5").assert().success().stdout("*\n**\n***\n****\n*****\n");
}