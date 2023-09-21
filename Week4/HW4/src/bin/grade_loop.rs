//Problem 2.1
fn score_to_grade_1(s: &[i64]) -> Vec<&str>{
    let mut new_s = Vec::with_capacity(s.len());
    #[allow(unused_comparisons)] //use to disable warning of 17th line
    for i in 0..s.len() {
        let p = s[i];
        
        if p >= 95 && p <= 100 {
            new_s.push("A+");
        } else if p >= 81 && p <= 94 {
            new_s.push("A");
        } else if p >= 71 && p <= 80 {
            new_s.push("B");
        } else if p >= 61 && p <= 70 {
            new_s.push("C");
        } else if p >= 50 && p <= 60 {
            new_s.push("D");
        } else if p >= 0 && p <= 49{
            new_s.push("F");
        } else {
            new_s.push("invalid") //for error handling if input is > 100 or < 0 
        }
    }
    new_s
}

fn main() {
    let score = [50, 30, 60, 89, 71];
    let result = score_to_grade_1(&score);

    println!("Score: {:?}", score);
    println!("Result: {:?}", result);
}

#[test]
fn test_grading() {
    assert_eq!(score_to_grade_1(&[85, 92, 76, 60, 45]), ["A", "A", "B", "D", "F"]);
    assert_eq!(score_to_grade_1(&[-1, 0, 10, 50]), ["invalid", "F", "F", "D"]);
}