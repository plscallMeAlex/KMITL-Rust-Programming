fn main() {
    let nm:Vec<String> = std::env::args().collect();
    if nm.len() != 2 {
        println!("Nothing");
    }
    let line:u32 = nm[1].parse().unwrap();
    let all_line = (2*line) -1;
    for i in 0..all_line {
        if i < line {
            for j in i..line-1 {
                print!(" ");
            }
            for k in 0..i+1 {
                print!("*");
            } 
        }  else {
            for j in 0..i-line+1 {
                print!(" ");
            } 
            for k in i..all_line {
                print!("*");
            }
        } println!("");
    }
}
