use assert_cmd::Command;

#[test]
fn main_circm(){
    let mut cmd = Command::cargo_bin("main").unwrap();
    cmd.arg("1");
    cmd.assert().success().stdout("c: 6.2832\n");
}

#[test]
fn main_run(){    
    let mut cmd = Command::cargo_bin("greet").unwrap();
    cmd.arg("Hei");
    cmd.assert().success().stdout("Hello, Hei\n");
}