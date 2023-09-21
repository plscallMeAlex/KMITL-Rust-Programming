use HW8::{savepoint, loadtopo, to_cartesian};
use std::{fs::File, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let args:Vec<_> = std::env::args().collect();
    if args.len() < 3 {eprintln!("use: {} <input_file> <output_file>",args[0]);return Ok(())}    
    
    let inp = &args[1];
    let out = &args[2];
    
    let fst = loadtopo(File::open(inp).expect("Fail to read file"))?;
    let snd = to_cartesian(fst);
    
    let output_file = File::create(out)?;
    savepoint(output_file, snd)?;
    Ok(())
}

//change to polar to cartesian