fn main() {
    let range:Vec<String> = std::env::args().collect();
    if range.len() != 4 {
        println!("Input incorrect format");
        return;
        //check for how many arguments type in command line if not have 3 argument then it will end 
    }
    let mut start_Fahr:i32 = range[1].parse().unwrap_or(0);
    let end_Fahr:i32 = range[2].parse().unwrap_or(0);
    let diff:i32 = range[3].parse().unwrap_or(0);
    //define variable from each input in command line
    
    println!("{:>5} {:>9}","Fahr", "Celcius");
    let amount = (end_Fahr - start_Fahr)/diff;
    if amount > 0 {
        for i in 0..amount+1 {
            let celcius: f32 = temp_tables::ftoc(start_Fahr);
            println!("{:>4.1}   {:>7.1}", start_Fahr, celcius);
            start_Fahr += diff
            //from small number to big number
        }
    }else if amount < 0 {
        for i in 0..amount.abs()+1 {
            let celcius: f32 = temp_tables::ftoc(start_Fahr);
            println!("{:>4.1}   {:>7.1}", start_Fahr, celcius);
            start_Fahr -= diff
            //from big number to small number
        }
    }
}
