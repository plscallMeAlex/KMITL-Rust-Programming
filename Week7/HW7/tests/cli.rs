use assert_cmd::Command;

#[test]
fn test01() {
    let mut cmd = Command::cargo_bin("HW7").unwrap();
    let a = ["2", "4", "1", "9", "7"];
    for i in a{
        cmd.arg(i);
    }
    cmd.assert().success().stdout("[1, 2, 4, 7, 9]\n[9, 7, 4, 2, 1]\n");
}

#[test]
fn test0102() {
    let mut cmd = Command::cargo_bin("Q1_2").unwrap();
    let a = ["2", "4", "1", "9", "7"];
    for i in a{
        cmd.arg(i);
    }
    cmd.assert().success().stdout("[1, 2, 4, 7, 9]\n[9, 7, 4, 2, 1]\n"); 
}

//test for Q2-1 and Q2-2
#[test]
fn test02() {
    let q2 = ["Q2_1", "Q2_2"];
    for i in q2 {
        let mut cmd = Command::cargo_bin(i).unwrap();
        let inp = ["1", "2", "3", "3", "4", "5", "4", "1", "4", "2", "4", "7", "5", "2", "5", "1", "4", "0", "-2", "-4"];
        for j in inp{
            cmd.arg(j);
        }
        cmd.assert().success().stdout("Asc: [(-2.0, -4.0), (1.0, 2.0), (3.0, 3.0), (4.0, 0.0), (4.0, 1.0), (4.0, 2.0), (4.0, 5.0), (4.0, 7.0), (5.0, 1.0), (5.0, 2.0)]\nDes: [(5.0, 2.0), (5.0, 1.0), (4.0, 7.0), (4.0, 5.0), (4.0, 2.0), (4.0, 1.0), (4.0, 0.0), (3.0, 3.0), (1.0, 2.0), (-2.0, -4.0)]\n");
    }
}