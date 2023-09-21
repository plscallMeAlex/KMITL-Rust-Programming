fn catesian(a: Vec<u64>, b: Vec<u64>) -> Vec<(u64, u64)> {
    let mut resuk = Vec::new();
    for word1 in a {
        for word2 in &b {
            resuk.push((word1, *word2));
        }
    }
    resuk
}

#[test]
fn teee() {
    assert_eq!(
        catesian(vec![1, 2, 3], vec![21, 12]),
        [(1, 21), (1, 12), (2, 21), (2, 12), (3, 21), (3, 12)]
    );
}
//Catesian Product
//V1 = [1, 2, 3]
//V2 = [21, 12]
//re = [(1, 21), (1, 21), (2, 21), (2, 12), (3, 21), (3, 12)]

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// fn join_strings(st:Vec<str>, ar:&str) -> String {
//     if st < 2 {0};
//     let mut new = String::new();
//     for i in st.iter().next() {
//         new.push_str(&i);
//         new.push_str(&ar);
//     }
//     new
// }

// #[test]
// fn test_join_strings() {
//     assert_eq!(join_strings(&[], ","), "");
//     assert_eq!(join_strings(&["C"], ","), "C");

//     let pat = ["C", "Rust", "C++", "Python"];
//     assert_eq!(join_strings(&pat, ", "), "C, Rust, C++, Python");
//     assert_eq!(join_strings(&pat, ";;"), "C;;Rust;;C++;;Python");
// }

pub fn triangle_patterns() {
    let inp = 5;
    if inp < 0 {
        println!("Input Some num");
    } else {
        let line = inp;
        for eline in 0..line + 1 {
            for estar in 0..eline + 1 {
                print!("*");
            }
            println!();
        }
    }
}

pub fn count_negative(v: &[i64]) -> usize {
    let mut count = 0;
    for co in v {
        if co < &0 {
            count += 1
        } else {
            continue;
        }
    }
    count
}

pub fn doubles(v: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    if v.len() == 0 {
        return result;
    }
    for num in v {
        result.push(num * 2);
    }
    result
}

pub fn quote_list(v: &[&str], c: char) -> Vec<String> {
    let mut resuk: Vec<String> = Vec::new();
    for word in v {
        resuk.push(c.to_string() + word + &c.to_string());
    }
    resuk
}

pub fn fahr_to_cel_v(f: &[i64]) -> Vec<f64> {
    let mut resuk: Vec<f64> = Vec::new();
    for num in f {
        let convert = ((5. / 9.) * (*num as f64 - 32.));
        resuk.push(convert);
    }
    resuk
}

pub fn reverse(v: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    if v.len() == 0 {
        return v.to_vec();
    }
    let mut n = v.len();
    for i in 0..v.len() {
        result.push(v[n - 1]);
        n -= 1
    }
    result
}
// #[test]
// fn testdfd() {
//     assert_eq!(quote_list((v), c))
// }

// fn min(v:&[i64]) -> i64 {
//     let mut a = v.iter();
//     let mut x;
//     while let Some(nv) = a.next() {
//         if *nv < x {
//             x = *nv;
//         }
//         else {
//             x;
//             continue;
//         }
//     }
//     x
// }

// fn max(v:&[i64]) -> i64 {
//     let mut a = v.iter();
//     let mut x;
//     if let Some(r) = a.next() {
//         while let Some(q) = a.next() {
//             if q < x {
//                 x = q
//             }
//             else {
//                 continue;
//             }
//         }
//     }
//     else {
//         0;
//     }
//     x
// }

pub fn quote_list_r(v: &[&str], c: char) -> Vec<String> {
    if v.len() == 0 {
        return Vec::new();
    } else {
        let mut result = quote_list_r(&v[0..v.len() - 1], c);
        result.push(c.to_string() + &v[v.len() - 1].to_string() + &c.to_string());
        result
    }
}

pub fn count_digit(inp: &str) -> usize {
    let mut result = 0;
    for i in inp.chars() {
        if i.is_digit(10) {
            result += 1
        }
    }
    result
}

#[test]
fn test_digits_count1() {
    assert_eq!(count_digit(""), 0);
    assert_eq!(count_digit("abcd"), 0);
    assert_eq!(count_digit("ab12xy5 7x83y5z"), 7);
}

fn count_digit_r(v: &str) -> usize {
    if let Some(x) = v.chars().next() {
        if x.is_digit(10) {
            let result = count_digit_r(&v[1..]) + 1;
            result
        } else {
            let result = count_digit_r(&v[1..]);
            result
        }
    } else {
        return 0;
    }
}

#[test]
fn test_digits_counsdst1() {
    assert_eq!(count_digit_r(""), 0);
    assert_eq!(count_digit_r("abcd"), 0);
    assert_eq!(count_digit_r("ab12xy5 7x83y5z"), 7);
}

fn non_negatives(inp: &[f64]) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    if inp.len() == 0 {
        return result;
    } else {
        for i in inp {
            if i >= &0. {
                result.push(*i);
            }
        }
    }
    result
}

#[test]
fn test_extract_non_negatives() {
    assert_eq!(non_negatives(&[]), []);
    assert_eq!(
        non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
        [0.8, 1.6, 10.5].to_vec()
    );
}

pub fn arrow_right(inp: u64) -> String {
    let mut result = String::new();
    let all_line = (2*inp)-1;
    let line = inp;
    for i in 1..=line {
        for pword in 1..=i {
            result.push('*');
        }
        result.push('\n');
    }
    for k in line..=all_line {
        for pword in 1..=all_line - k {
            result.push('*');
        }
        result.push('\n');
    }
    result
}

pub fn count_vowels_r(v:&str) -> usize {
    let vowels = "aeiouAEIOU";
    if let Some(x) = v.chars().next(){
        if vowels.contains(x){
            let y = count_vowels_r(&v[1..]) +1;
            return y
        } else {
            let y = count_vowels_r(&v[1..]);
            return y
        }
    } else {
        return 0;
    }
}