//Problem 3.1
fn make_arrow1(st: i64) -> String {
    let st = st.abs();
    match st {
        0 => String::new(),//handling for if input 0 it will return nothing
        _ => {
            let all_line = (2 * st) -1; //to know how much line need to print
            let mut arrow_pattern = String::new();
        
            for i in 0..all_line {
                #[allow(unused_variables)]
                if i < st { //for step up
                    for x in 0..i+1 { //loop to push star into new  in each line
                        arrow_pattern.push('*');
                    }
                } else {
                    for x in i..all_line {
                        arrow_pattern.push('*'); //middle 
                    }
                } 
                if i < all_line -1 {
                    arrow_pattern.push('\n'); //step down

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

    match input[1].parse::<i64>() { //error handling
        Ok(input_num) => {
            let result = make_arrow1(input_num.abs());
            println!("{}",result);
        }
        Err(_) => {
            println!("Error")
        }
    }
}
//to run this use this pattern of the following command: cargo run --bin arrow 5 (u can change from 5 to another number)
#[test]
fn test_arrow() {
    assert_eq!(make_arrow1(5), ("*\n**\n***\n****\n*****\n****\n***\n**\n*"));
    assert_eq!(make_arrow1(0), (""));
    assert_eq!(make_arrow1(-1), ("*"));
}