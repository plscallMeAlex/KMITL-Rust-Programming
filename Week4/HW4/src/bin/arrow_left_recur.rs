//Program 3.3 left
fn _arrow(li: usize, all_line: usize, mut arrow_pattern: String, eline: usize, mut eloop: usize) -> String {
    if eline <= all_line {
        if eline <= li { // step up
            if eloop < (li - eline) { //for print blank ex:input =5 the first line the blank will be push for 5-line = 5-1 =4 like this
                arrow_pattern.push(' ');
                eloop += 1;
                return _arrow(li, all_line, arrow_pattern, eline, eloop);
            } 
            else {
                if eloop < li {
                    arrow_pattern.push('*'); //will push at the end before it start new line
                    eloop += 1;
                    return  _arrow(li, all_line, arrow_pattern, eline, eloop);
                }
            }
        } else {
            if eloop < (eline - li) { //step down
                arrow_pattern.push(' ');
                eloop += 1;
                return _arrow(li, all_line, arrow_pattern, eline, eloop); //return again for push blank again
            } else {
                if eloop < li {
                    arrow_pattern.push('*');//it will push star increased
                    eloop += 1;
                    return _arrow(li, all_line, arrow_pattern, eline, eloop);
                }
            }
        }
        if eline < all_line {
            arrow_pattern.push('\n');//for at the last i won't newline
        }
        return _arrow(li, all_line, arrow_pattern, eline + 1, 0);
    }
    arrow_pattern
}

fn make_arrow2(line: i64) -> String {
    let line = line.abs();
    match line { //error handling
        0 => String::new(),
        _ => {
            let all_line = (2 * line) - 1;
            let arrow_pattern = String::new();
            let eline = 1;
            _arrow(line as usize, all_line as usize, arrow_pattern, eline, 0)
        }
    }
}

fn main() {
    let arrow: i64 = 5;
    let result = make_arrow2(arrow);
    println!("{}", result);
}

#[test]
fn test_arrow() {
    assert_eq!(make_arrow2(5), ("    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *"));
    assert_eq!(make_arrow2(0), (""));
    assert_eq!(make_arrow2(-1), ("*"));
}