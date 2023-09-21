// fn join_strings(s:&[&str], y:&str) -> String {
//     let mut comb= String::new();
//     let mut word = s.iter();
//     let eword = word.next();

//     if s.is_empty() {
//         return String::new()
//     }

//     while let Some(eword) = s.iter().next() {
//         comb.push("{}", eword);
//         comb.push("{}",&y);
//     }

//     comb
// }

fn join_strings(s: &[&str], y: &str) -> String {
    let mut comb = String::new();
    let mut word = s.iter();

    if let Some(eword) = word.next() {
        comb.push_str(eword);
    }

    for eword in word {
        comb.push_str(y);
        comb.push_str(eword);
    }

    comb
}

#[test]
fn test_join_strings() {
    assert_eq!(join_strings(&[], ","), "");
    assert_eq!(join_strings(&["C"], ","), "C");

    let patterns = ["C", "Rust", "C++", "Python"];
    assert_eq!(join_strings(&patterns, ", "), "C, Rust, C++, Python");
    assert_eq!(join_strings(&patterns, ";;"), "C;;Rust;;C++;;Python");
}

fn join_numbers(nb: &[i64], s: &str) -> String {
    let mut ewe = String::new();
    let mut num = nb.iter().map(|&n| n.to_string());

    if let Some(enumr) = num.next() {
        ewe.push_str(&enumr);
    }

    for enumr in num {
        ewe.push_str(s);
        ewe.push_str(&enumr);
    }

    ewe
}

#[test]
fn test_join_numbers() {
    assert_eq!(join_numbers(&[], ","), "");
    assert_eq!(join_numbers(&[25], ","), "25");

    let patterns = [5, 10, -1, 2];
    assert_eq!(join_numbers(&patterns, ", "), "5, 10, -1, 2");
    assert_eq!(join_numbers(&patterns, ";;"), "5;;10;;-1;;2");
}

fn pack_number_tuples(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    let max_len = a.len().max(b.len());
    let mut result = Vec::with_capacity(max_len);

    for i in 0..max_len {
        let num_a = a.get(i).cloned().unwrap_or(0);
        let num_b = b.get(i).cloned().unwrap_or(0);
        result.push((num_a, num_b));
    }

    result
}

#[test]
fn test_pack_number_tuples() {
    assert_eq!(pack_number_tuples(&[], &[]), vec![]);
    assert_eq!(pack_number_tuples(&[1], &[]), vec![(1, 0)]);
    assert_eq!(pack_number_tuples(&[], &[2, 3]), vec![(0, 2), (0, 3)]);
    assert_eq!(
        pack_number_tuples(&[5, 1, 4], &[2, 3]),
        vec![(5, 2), (1, 3), (4, 0)]
    );
}

fn pack_number_tuples_s(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    let max_len = a.len().min(b.len());
    let mut result = Vec::with_capacity(max_len);

    for i in 0..max_len {
        let num_a = a[i];
        let num_b = b[i];
        result.push((num_a, num_b));
    }

    result
}

#[test]
fn test_join_stringssss() {
    assert_eq!(pack_number_tuples_s(&[], &[]), []);
    assert_eq!(pack_number_tuples_s(&[1], &[]), []);
    assert_eq!(pack_number_tuples_s(&[], &[2, 3]), []);
    assert_eq!(pack_number_tuples_s(&[5, 1, 4], &[2, 3]), [(5, 2), (1, 3)]);
}
