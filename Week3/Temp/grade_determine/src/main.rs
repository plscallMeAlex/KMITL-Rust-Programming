use std::env;

fn main() {
    let g: Vec<String> = std::env::args().collect();
    if g.len() < 2{ //if 
        println!("Input any score"); 
        return;
    }
    let gpa: Result<u32, _> = g[1].parse();
    match gpa {
        Ok(gpa) =>{
             if gpa >= 95 && gpa <= 100 {
                println!("Excellent with A+");
            } else if gpa >= 81 && gpa <= 94 {
                println!("A");
            } else if gpa >= 71 && gpa <= 80 {
                println!("B");
            } else if gpa >= 61 && gpa <= 70 {
                println!("C");
            } else if gpa >= 50 && gpa <= 60 {
                println!("D");
            } else if gpa >= 0 && gpa <= 49 {
                println!("Failed with F");
            } else {
                println!("Invalid score");    
            } 
        } Err(_) =>{
            println!("Invalid score");
        }
    }
}
