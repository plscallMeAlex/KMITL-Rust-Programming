fn main() {
    let nm: Vec<String> = std::env::args().collect();
    if nm.len() < 2 {
        println!();
        return;
    }
    let line: u32 = nm[1].parse().unwrap();
    if line == 0{
        println!("Nothing Happened");
        return;
    } else {
        let all_line: u32 = (2 * line) - 1;
        for i in 0..all_line {
            if i < line {
                for j in 0..i+1 {
                    print!("*");
                }
            }else {
                for j in i..all_line {
                    print!("*");
                }
            } println!();
        }
    }
}
