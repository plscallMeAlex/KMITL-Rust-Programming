fn main() {
    let nm: Vec<String> = std::env::args().collect();
    if nm.len() < 2 {
        println!();
        return;
    }//not input anything
    let line: u32 = nm[1].parse().unwrap_or(0);
    if line == 0{
        println!();
        return; //input value = o will do nothing
    } else {
        let all_line: u32 = (2 * line) - 1; //find for how many lines need to print and for loop
        for i in 0..all_line {
            if i < line {
                for j in 0..i+1 {
                    print!("*"); //when i value is less than input value it will print * in the same line in range j(incresing slope)
                }
            }else {
                for j in i..all_line {
                    print!("*");// when it in the middle line of all line in will go in this case print * in the range that decrese everyloop cuz i increse and then the range between i and all line also decrease
                }
            } println!();
        }
    }
}
