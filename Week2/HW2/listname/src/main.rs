use std::env;

fn main() {
    let name: Vec<String> = env::args().collect();
    if name.len() < 2 {
        println!("Player 1: {}","N/A");
        println!("Player 2: {}","N/A");
    }else if name.len() < 3 && name.len() > 1{
        println!("Player 1: {}",name[1]);
        println!("Player 2: {}","N/A");
    }else{
        println!("Player 1: {}",name[1]);
        println!("Player 2: {}",name[2]);
    };
}
