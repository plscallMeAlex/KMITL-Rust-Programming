pub fn ftoc(start_fahr: i32) -> f32 {
    (5.0/9.0)*(start_fahr as f32 - 32.0)
} //function change temperatures

fn main() {
    let range:Vec<String> = std::env::args().collect();
    if range.len() != 4 {
        println!("Input incorrect format");
        return;
        //check for how many arguments type in command line if not have 3 argument then it will end 
    }
    let mut start_fahr:i32 = range[1].parse().unwrap_or(0);
    let end_fahr:i32 = range[2].parse().unwrap_or(0);
    let diff:i32 = range[3].parse().unwrap_or(0);
    //define variable from each input in command line

    println!("{:>5} {:>9}","fahr", "celcius");
    let amount = (end_fahr - start_fahr)/diff;
    if amount > 0 {
        for _i in 0..amount+1 {
            let celcius: f32 = ftoc(start_fahr);
            println!("{:>4.1}   {:>7.1}", start_fahr, celcius);
            start_fahr += diff
            //from small number to big number
        }
    }else if amount < 0 {
        for _i in 0..amount.abs()+1 { //need to how much times need to loop so must use abs() to make it positive
            let celcius: f32 = ftoc(start_fahr);
            println!("{:>4.1}   {:>7.1}", start_fahr, celcius);
            start_fahr -= diff
            //from big number to small number
        }
    }
}
