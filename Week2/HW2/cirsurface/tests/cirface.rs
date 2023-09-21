use assert_cmd::Command;

#[test]
fn cir_area01(){
    let mut cmd = Command::cargo_bin("cirsurface").unwrap();
    cmd.arg("1").assert().success();
}

#[test]
fn cir_area02(){
    let mut cmd = Command::cargo_bin("cirsurface").unwrap();
    cmd.assert().failure();
}

#[test]
fn cir_area03(){
    let mut cmd = Command::cargo_bin("cirsurface").unwrap();
    cmd.arg("10").assert().success().stdout("The Area of the circle is: 314.159\n");
}
