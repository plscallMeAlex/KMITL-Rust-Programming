use HW8::{savedt, load, to_polar};
use std::{fs::File, error::Error};

//for q2.1
fn main() -> Result<(), Box<dyn Error>>{
    let args:Vec<_> = std::env::args().collect();
    if args.len() < 3 {eprintln!("use: {} <input_file> <output_file>",args[0]);return Ok(())}    
    
    let inp = &args[1];
    let out = &args[2];
    
    let fst = load(File::open(inp).expect("Fail to read file"))?;
    let snd = to_polar(fst);
    
    let output_file = File::create(out)?;
    savedt(output_file, snd)?;
    Ok(())
}
