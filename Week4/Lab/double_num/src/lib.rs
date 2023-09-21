// use std::fmt::format;

// fn count_negative(v: &[i64]) -> usize {
//     v.iter().filter(|&&x| x < 0).count()
// }

// fn doubles(v: &[i64]) -> Vec<i64> {
//     // v.iter().map(|x| x * 2).collect()
    
//     // for i in 0..v.len() {
//     //     let mut _x = v[i] * 2;
//     //     new_v.push(_x);
//     // }
//     // return new_v.to_vec();
//     let mut new_v = Vec::with_capacity(v.len());
//     fn double2(v: &[i64], new_v: &mut Vec<i64>, index: usize) {
//         if index < v.len() {
//             new_v.push(v[index] * 2);
//             double2(v, new_v, index + 1);
//         }
//     }
//     double2(v, &mut new_v, 0);
//     new_v
// }  

// #[test]
// fn test_counting() {
//     assert_eq!(count_negative(&[]), 0);
//     assert_eq!(count_negative(&[1, 2, -3, 4, -6, 7]), 2);
//     assert_eq!(doubles(&[1, 2, 3, 4, 5]), [2, 4, 6, 8, 10]);
// }

fn quote(s: &str, c: char) -> String {
    format!("{}{}{}", c, s, c)
}

fn quote_list(v: &[&str], c: char) -> Vec<String> {
    // v.iter().map(|s| format!("{}{}{}", c, s, c)).collect()
    
    // let mut new_v = Vec::with_capacity(v.len());
    // for &s in v {
    //     let mut _x = format!("{}{}{}", c, s, c);
    //     new_v.push(_x);
    // }
    // new_v
    let mut new_v = Vec::with_capacity(v.len());
    fn quote_list2(v: &[&str], c: char, new_v: &mut Vec<String> index: usize){
        if index < v.len() {
            format!("{}{}{}", c, s, c);
            new_v.push(v[index])
        }
    } //recursion
}

#[test] //unit test
fn test_quotes() {
    assert_eq!(quote("abcd", '*'), "*abcd*");
    assert_eq!(quote_list(&[""; 0], '*'), &[""; 0]);
    assert_eq!(
        quote_list(&["abcd", "xyz"], '*'),
        ["*abcd*", "*xyz*"]
    );
    assert_eq!(quote_list(&["sas", "dddw"], '*'),
        ["*sas*", "*dddw*"]
    );
}