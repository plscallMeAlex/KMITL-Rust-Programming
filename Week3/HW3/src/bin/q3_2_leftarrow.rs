fn main() {
    let nm:Vec<String> = std::env::args().collect();
    if nm.len() < 2 {
        println!("");
        return;//same as 3.1 that filtered if the user not something
    }
    let line:u32 = nm[1].parse().unwrap_or(0);
    if line == 0 {
        println!("");
        return; //if input value = 0 it also return nothing only blank line
    } else {
        let all_line = (2*line) -1;
        for i in 0..all_line {
            if i < line {
                for _j in i..line-1 {
                    print!(" ");
                }
                for _k in 0..i+1 {
                    print!("*");
                } //same as 3.1 but it difference when this 3.2 will loop print space_bar and then print * at the end(increasing slope)
            }  else {
                for _j in 0..i-line+1 {
                    print!(" ");
                } //cus we have only half of all line so we need to subtract i value because if i value is higher it print blank space in the same line much more 
                for _k in i..all_line {
                    print!("*");
                }// decreasing slope if range from i to all_line is increase then it will decresing of printing * in each line 
            } println!("");
        }  
    }
}
