//Problem 3.2
fn make_arrow2(le:i64) -> String {
    match le {
        0 => String::new(),
        _ => {
            let le = le.abs();
            let all_line = (2 * le) -1;
            let mut arrow_pattern = String::new();
        
            for i in 0..all_line {
                if i < le {
                    for _j in i..le-1 {
                        arrow_pattern.push(' ');
                    }
                    for _k in 0..i+1 {
                        arrow_pattern.push('*');
                    } //same as 3.1 but it difference when this 3.2 will loop print space_bar and then print * at the end(increasing slope)
                }  else {
                    for _j in 0..i-le+1 {
                        arrow_pattern.push(' ');
                    } //cus we have only half of all line so we need to subtract i value because if i value is higher it print blank space in the same line much more 
                    for _k in i..all_line {
                        arrow_pattern.push('*');
                    }// decreasing slope if range from i to all_line is increase then it will decresing of printing * in each line 
                } 
                if i < all_line -1 {
                    arrow_pattern.push('\n');
                }
            }
            arrow_pattern
        }
    }
    
}

fn main() {
    let input:Vec<String> = std::env::args().collect();
    if input.len() != 2 {
        println!("Input how big of arrow");
        return;
    }
    
    match input[1].parse::<i64>() {
        Ok(input_num) => {
            let result = make_arrow2(input_num.abs());
            println!("{}",result);
        }
        Err(_) => {
            println!("Error")
        }
    }
}

#[test]
fn test_arrow() {
    assert_eq!(make_arrow2(5), ("    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *"));
    assert_eq!(make_arrow2(0), (""));
    assert_eq!(make_arrow2(-1), ("*"));
}