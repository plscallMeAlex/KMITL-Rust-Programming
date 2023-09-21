//Problem 2.2
fn score_to_grade_2(s: &[i64]) -> Vec<&'static str>{
    let mut new_s = Vec::with_capacity(s.len());
    stog(s, &mut new_s, 0); //call function and it will start here
    new_s
}

fn stog(s: &[i64], new_s: & mut Vec<&str>, round: usize){ //recursion
    match s {
        [] => {},
        _ => {
            if round < s.len() { 
                new_s.push(convert_grade(s[round]));
                stog(s, new_s, round+1)
            }
        }       
    }
}

fn convert_grade(p: i64) -> &'static str {
    if p >= 95 && p <= 100 {
        "A+"
    } else if p >= 81 && p <= 94 {
        "A"
    } else if p >= 71 && p <= 80 {
        "B"
    } else if p >= 61 && p <= 70 {
        "C"
    } else if p >= 50 && p <= 60 {
        "D"
    } else if p <= 49 && p >= 0{
        "F"
    } else {
        "Invalid Score"
    }

}

fn main() {
    let score = [60, 50, 10, 90, 80];
    let result = score_to_grade_2(&score);

    println!("Score: {:?}", score);
    println!("Grade: {:?}", result);
}

#[test]
fn test_grading() {
    assert_eq!(score_to_grade_2(&[40, 60, 80, 100]), ["F", "D", "B", "A+"]);
    assert_eq!(score_to_grade_2(&[0, 50, -10, -20]), ["F", "D", "Invalid Score", "Invalid Score"]);     
}