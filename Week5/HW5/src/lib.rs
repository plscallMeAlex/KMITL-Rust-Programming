use std::vec;

// //Q1.1
fn count_vowels(ct: &str) -> usize {
    let ip = ct.to_lowercase();
    let ip2 = ip.chars();
    let vowels = ['a', 'e', 'i', 'o', 'u']; 
    let mut count = 0; //count number of vowel in the word 
    for char in ip2 { //check for each character if it have any of vowels the count will be increase
        if vowels.contains(&char) {
            count += 1;
        }
    }
    count //return value out of function
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("0"), 0);
}

//-----------------------------------------------------------------------------------------------------------------------------------
//Q1.2
fn count_vowel_r(ct: &str, index: usize) -> usize {
    if index >= ct.len() {
        return 0;
    }

    let char_lower = ct //get a word from the input and split it into char lower it to check if its the same as var_name (vowels)
        .chars()
        .nth(index)
        .unwrap()
        .to_lowercase()
        .next()
        .unwrap();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let is_vowel = vowels.contains(&char_lower);
    let count = if is_vowel { 1 } else { 0 }; // if it contain vowels it will return 1 if not return 0 instead
    count + count_vowel_r(ct, index + 1)
}

fn count_vowel(ct: &str) -> usize {
    count_vowel_r(ct, 0)
}

#[test]
fn test_vowels_count_r() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("0"), 0);
}

//-----------------------------------------------------------------------------------------------------------------------------------
//Q1.3
fn count_vowels_v2(cv: &str) -> Vec<(String, usize)> { //fokr returning 2 variable in vector of tuple 
    // let kham:Vec<&str> = cv.split_whitespace().collect();
    let vowels = "aeiouAEIOU";
    let mut op: Vec<(String, usize)> = Vec::new();
    for ck_word in cv.split_whitespace() { //split the white_space and will get how many word inside of input
        let mut count = 0;
        for i in ck_word.chars() {
            if vowels.contains(i) { //checking char in each word that if contain some of vowels count will increase
                count += 1;
            }
        }
        op.push((ck_word.to_string(), count)); //push a tuple into vector 
    }
    op
}

#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
            ("7x8U3y5z".to_string(), 1)   // 'U'
        ]
    );
    assert_eq!(count_vowels_v2("ssdsaeko"), [("ssdsaeko".to_string(), 3)]);
    assert_eq!(count_vowels_v2("0127437e gthgfbdcsds3"), [("0127437e".to_string(), 1), ("gthgfbdcsds3".to_string(), 0)]);
}

//-----------------------------------------------------------------------------------------------------------------------------------
//Q2.1
fn split_grades(grd: &[String]) -> (Vec<String>, Vec<String>) {
    let mut new_failur: Vec<String> = Vec::new(); //need to have 2 vector cus it seperate between pass and fail(below grade D)
    let mut new_pass: Vec<String> = Vec::new();
    for i in grd {
        match i.to_uppercase().as_str() {
            "A+" | "A" | "B" | "C" => new_pass.push(i.to_string()), //if it match some grade in this some it will put itself to new vec
            "D" | "F" => new_failur.push(i.to_string()),
            _ => (), //handling another input that not is a grade
        }
    }
    (new_pass, new_failur) //return a tuple value 
}

#[test]
fn test_pass_fail() {
    assert_eq!(
        split_grades(&[
            "B".to_string(),
            "F".to_string(),
            "A+".to_string(),
            "D".to_string(),
            "C".to_string()
        ]),
        (
            vec!["B".to_string(), "A+".to_string(), "C".to_string()],
            vec!["F".to_string(), "D".to_string()]
        )
    );
    assert_eq!(
        split_grades(&[
            "C".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
            "".to_string()
        ]),
        (vec!["C".to_string()], vec!["F".to_string()])
    );
}

//-----------------------------------------------------------------------------------------------------------------------------------
//Q2.2.to
fn check_grade(gd: u32) -> String { //another function for checking grades 
    let mut result = "";
    if gd >= 95 && gd <= 100 {
        result ="A+";
    } else if gd >= 81 && gd <= 94 {
        result ="A";
    } else if gd >= 71 && gd <= 80 {
        result ="B";
    } else if gd >= 61 && gd <= 70 {
        result ="C";
    } else if gd >= 50 && gd <= 60 {
        result ="D";
    } else if gd >= 0 && gd <= 49 {
        result ="F";
    } else {
        result ="Invalid";
    }

    result.to_string()
}

fn split_scores(sc: Vec<u32>) -> (Vec<(String, usize)>, Vec<(String, usize)>) { //main function is here
    let mut pass: Vec<(String, usize)> = Vec::new();
    let mut fail: Vec<(String, usize)> = Vec::new();
    for g in sc {
        let a: String = check_grade(g);
        if a.as_str() == "A+" || a == "A" || a == "B" || a == "C"{
            pass.push((a, (g).try_into().unwrap())); //fileter a score, push it to the vector and turn into usize on var_name (g) and return a grade to a grade into the same vector (return a tuple of (Grade,Score))
        }else if a == "D" || a == "F"{
            fail.push((a, (g).try_into().unwrap()))
        } else {
            fail.push((a, (g).try_into().unwrap()))
        }
    }
    (pass, fail)
}
#[test]
fn test_split_scores() {
    assert_eq!(split_scores((&[]).to_vec()), (vec![], vec![]));
    assert_eq!(
        split_scores((&[75, 42, 98, 54, 63]).to_vec()),
        (
            vec![
                ("B".to_string(), 75),
                ("A+".to_string(), 98),
                ("C".to_string(), 63)
            ],
            vec![("F".to_string(), 42), ("D".to_string(), 54)]
        )
    );
    assert_eq!(split_scores((&[45, 80, 101]).to_vec()), (vec![("B".to_string(), 80)], vec![("F".to_string(), 45), ("Invalid".to_string(), 101)]));
}

//-----------------------------------------------------------------------------------------------------------------------------------
//Q3.1
fn extract_quoted_words(wd: &str) -> Vec<String> {
    let mut po: Vec<String> = Vec::new();
    for w in wd.split_whitespace() { //split a word 
        if w.starts_with('*') && w.ends_with('*') { //check is it correct pattern that start with star and end with star 
            let x = w.trim_start_matches("*").trim_end_matches("*").to_string(); //remove star out and push it to vector 
            po.push(x.to_string())
        }
    }
    po
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        ["", "C++", "Python"]
    );
    assert_eq!(extract_quoted_words("fdfdfee *[][[* dd* *232*"), ["[][[", "232"]);
}

//-----------------------------------------------------------------------------------------------------------------------------------
//Q3.2
fn extract_quoted_words_r(wd: &str) -> Vec<String> {
    let mut po: Vec<String> = Vec::new();
    let words: Vec<&str> = wd.split_whitespace().collect();//split a word and return to vector
    if words.len() > 1{ //if it have a input in vector will go in this condition
        if words[0].starts_with('*') && words[0].ends_with('*') { //same as 3.1
            let x = words[0].trim_start_matches("*").trim_end_matches("*").to_string(); //same as 3.1 and push to vector
            po.push(x.to_string()) 
        }
    po.extend(extract_quoted_words_r(&words[1..].join(" ")));
    }
    po
}

#[test]
fn test_extract_quoted_words_r() {
    assert_eq!(extract_quoted_words_r(""), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words_r("C ** *C++* *Java *Python* Rust*"),
        ["", "C++", "Python"]
    );
    assert_eq!(
        extract_quoted_words_r("P ** *u* *yyy* *ppp "),
        ["", "u", "yyy"]
    );
    assert_eq!(extract_quoted_words_r("33 *3eh *fof* eder*"), ["fof"]);
}
