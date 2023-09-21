use assert_cmd::Command;
use std::fs;
use std::error::Error;


#[test]
fn test01() -> Result<(), Box<dyn Error>> {
    let expected_file = "tests/expected/output.txt";
    let expected = fs::read_to_string(expected_file)?;

    let mut cmd = Command::cargo_bin("q2_temp").unwrap();
    cmd.arg("0").arg("300").arg("20");
    let output = cmd.output()?.stdout;

    let output_str = String::from_utf8_lossy(&output).to_lowercase();

    assert_eq!(expected.to_lowercase(), output_str);

    Ok(()) ////this test is read from text file 
}


#[test]
fn test02() {
    let mut cmd = Command::cargo_bin("q2_temp").unwrap();
    cmd.assert().success().stdout("Input incorrect format\n");
}