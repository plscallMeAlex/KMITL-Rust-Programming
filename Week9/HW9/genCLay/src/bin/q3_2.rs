use std::error::Error;
use std::fs::File;
use genCLay::{loadfile, savetohtml2, cal_average_area_min_max};

fn main() -> Result<(), Box<dyn Error>> {
    let inp:Vec<String> = std::env::args().collect();
    if inp.len() != 3 {eprintln!("Enter wrong format!: cargo run [input_filename] [output_filename]"); return Ok(())}
    let file_name = format!("{}.csv",&inp[1]);
    let outputfile = format!("{}.html",&inp[2]);
    let openn = loadfile(File::open(file_name)?)?;
    let result = cal_average_area_min_max(&openn);
    let saving = savetohtml2(outputfile, result);
    Ok(())
}