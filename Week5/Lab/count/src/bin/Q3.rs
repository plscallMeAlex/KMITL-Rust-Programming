// pub fn count_digits(dg: Vec<String>) -> usize {
//     let mut digit_count = 0;

//     for s in dg {
//         for c in s.chars() {
//             if c.is_digit(10) {
//                 digit_count += 1;
//             }
//         }
//     }

//     digit_count
// }

// pub fn run() -> Result<(), Box<dyn std::error::Error>> {
//     let x = std::env::args().collect::<Vec<String>>();
//     println!("{:#?}", count_digits(x));
//     // do something in this function
//     Ok(())
// }

pub fn count_digits(dg: &str) -> usize {
    let digit_count = dg.chars().filter(|c| c.is_digit(10)).count();
    digit_count
}

pub fn run(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let digits_count = count_digits(input);
    println!("Input: {}", input);
    println!("Number of digits: {}", digits_count);
    // do something in this function
    Ok(())
}

fn main() {
    println!("Input: {}", input);
    println!("Number of digits: {}", digits_count);
}