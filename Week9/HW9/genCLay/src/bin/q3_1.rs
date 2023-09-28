use std::error::Error;
use std::fs::File;
use genCLay::{loadfile, savetohtml1, cal_average_area};

fn main() -> Result<(), Box<dyn Error>> {
    let inp:Vec<String> = std::env::args().collect();
    if inp.len() != 3 {eprintln!("Enter wrong format!: cargo run [input_filename] [output_filename]"); return Ok(())}
    let file_name = format!("{}.csv",&inp[1]);
    let outputfile = format!("{}.html",&inp[2]);
    let openn = loadfile(File::open(file_name)?)?;
    let result = cal_average_area(&openn);
    let saving = savetohtml1(outputfile, result);
    Ok(())
}