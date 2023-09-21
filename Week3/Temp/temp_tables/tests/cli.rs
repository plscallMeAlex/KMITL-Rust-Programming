use assert_cmd::Command;
use std::fs;
use std::error::Error;

#[test]
fn test01() -> Result<(), Box<dyn Error>> {
    let expected_file = "tests/output.txt";
    let expected = fs::read_to_string(expected_file)?;

    let mut cmd = Command::cargo_bin("temp_tables").unwrap();
    cmd.arg("0").arg("300").arg("20");
    cmd.assert().success().stdout(expected);

    Ok(())
}

#[test]
fn test02() {
    let mut cmd = Command::cargo_bin("temp_tables").unwrap();
    cmd.assert().success().stdout("Input incorrect format\n");
}