fn count_digits(input: &str) -> usize {
    let mut digit = 0;
    for c in input.chars(){
      if c.is_digit(10){
        digit +=1
      }
    }
    digit
}

#[test]
fn test_digits_count1() {
    assert_eq!(count_digits(""), 0);
    assert_eq!(count_digits("abcd"), 0);
    assert_eq!(count_digits("ab12xy5 7x83y5z"), 7);
}

fn count_digits_r(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut digit_count = 0;
    if input.chars().next().unwrap().is_digit(10) {
        digit_count = 1;
    }

    digit_count + count_digits(&input[1..])
}

#[test]
fn test_digits_count2() {
    assert_eq!(count_digits_r(""), 0);
    assert_eq!(count_digits_r("abcd"), 0);
    assert_eq!(count_digits_r("ab12xy5 7x83y5z"), 7);
}
// fn count_digits_v2(input: &str) -> Vec<(String, isize)> {
    //     let mut result = Vec::new();
    //     let store:Split<&str> = input.split(" ") ;
    //     for i in store {
        //         let mut digit = 0;
        //         for x in i.chars() {
            //             if x.is_digit(10) {
                //                 digit += 1
                //             }
                //         }
                //         result.push((i , digit));
                //     }
                //     result
                // }
                
                
use std::str::Split;
fn count_digits_v2(input: &str) -> Vec<(String, usize)> {
    let mut result = Vec::new();
    for word in input.split_whitespace() {
        let mut digit = 0;
        for d in word.chars() {
            if d.is_digit(10) {
                digit += 1;
        }

    }

    result.push((word.to_string(), digit));
    }
    result
}

#[test]
fn test_digits_count3() {
    assert_eq!(count_digits_v2(""), []);
    assert_eq!(
        count_digits_v2("ab12xy5 7x83y5z"),
        [
            ("ab12xy5".to_string(), 3), // '1', '2', '5'
            ("7x83y5z".to_string(), 4) // '7', '8', '3', '5'
        ]
    );
}

fn extract_non_negatives(input: &[f32]) -> Vec<f32>{
    let mut result = Vec :: new();
    for &num in input{
        if num > 0.0{
            result.push(num)
        }
    }
    result
}

#[test]
fn test_extract_non_negatives() {
    assert_eq!(extract_non_negatives(&[].to_vec()), [].to_vec());
    assert_eq!(
        extract_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5].to_vec()),
        [0.8, 1.6, 10.5].to_vec()
    );
}

fn extract_non_negatives_r(input: &[f32]) -> Vec<f32>{
    let mut result = Vec :: new();
    if input.is_empty(){
        return result
    }
    else if input[0] > 0.0 {
        result.push(input[0])
    }
    result.extend_from_slice(&extract_non_negatives_r(&input[1..]));

    result
}

#[test]
fn test_extract_non_negatives2() {
    assert_eq!(extract_non_negatives_r(&[].to_vec()), [].to_vec());
    assert_eq!(
        extract_non_negatives_r(&[0.8, -5.1, 1.6, -6.5, 10.5].to_vec()),
        [0.8, 1.6, 10.5].to_vec()
    );
}

fn split_non_negatives(input: &[f32]) -> (Vec<f32>, Vec<f32>){
    let mut positive = Vec :: new();
    let mut negative = Vec :: new();
    for &round in input{
        if round > 0.0{
            positive.push(round);
        }
        else if round < 0.0 {
            negative.push(round);
        }
    }
    return (positive, negative)
}

#[test]
fn test_split_non_negatives() {
    assert_eq!(split_non_negatives(&[]), (vec![], vec![]));
    assert_eq!(
        split_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
        (
            vec![0.8, 1.6, 10.5],
            vec![-5.1, -6.5]
        )
    );
}
