use std::error::Error;
use std::fs::File;
use genCLay::{loadfile, savefile1, cal_average_area};

fn main() -> Result<(), Box<dyn Error>> {
    let inp:Vec<String> = std::env::args().collect();
    if inp.len() != 3 {eprintln!("Enter wrong format!: cargo run [input_filename] [output_filename]"); return Ok(())}
    let file_name = format!("{}.csv",&inp[1]);
    let outputfile = format!("{}.csv",&inp[2]);
    let openn = loadfile(File::open(file_name)?)?;
    // println!("{:?}", openn);
    let result = cal_average_area(&openn);
    let saving = savefile1(File::create(outputfile)?, result)?;
    Ok(())
}