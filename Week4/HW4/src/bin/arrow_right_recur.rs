//Problem 3.3 right
fn _arrow(li: usize, all_line: usize, mut arrow_pattern: String, eline: usize, mut eloop: usize) -> String { //assign eline for counter each line and eloop is counter of looping
    if eline <= all_line {
        if eline <= li { //step up and middle
            if eloop < eline {
                arrow_pattern.push('*');//ex: if we stay on second floor it might be loop for 2 times until the condition is false
                eloop += 1;
                return _arrow(li, all_line, arrow_pattern, eline, eloop);
            }
        } else { //step down
            if eloop < (all_line - eline + 1) {
                arrow_pattern.push('*');
                eloop += 1;
                return _arrow(li, all_line, arrow_pattern, eline, eloop);
            }
        }
        if eline < all_line {
            arrow_pattern.push('\n');
        }
        return _arrow(li, all_line, arrow_pattern, eline + 1, 0);
    }
    arrow_pattern
}

fn make_arrow(line: i64) -> String {
    let line = line.abs();
    match line {
        0 => String::new(),
        _ => {
            let all_line = (2 * line) - 1;
            let arrow_pattern = String::new();
            let eline = 1;
            _arrow(line as usize, all_line as usize, arrow_pattern, eline, 0)//call fn
        }
    }
}

fn main() {
    let arrow: i64 = 5;
    let result = make_arrow(arrow);
    println!("{}", result);
}

#[test]
fn test_arrow() {
    assert_eq!(make_arrow(5), ("*\n**\n***\n****\n*****\n****\n***\n**\n*"));
    assert_eq!(make_arrow(0), (""));
    assert_eq!(make_arrow(-1), ("*"));
}
