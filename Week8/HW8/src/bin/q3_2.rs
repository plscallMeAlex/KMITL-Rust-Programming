use std::{fs::File, error::Error};
use HW8::{loadtopo, to_cartesian, saveashtml2};

fn main() -> Result<(), Box<dyn Error>> {
    let inp:Vec<String> = std::env::args().collect();
    if inp.len() < 3 {eprintln!("use: {} <input_file> <output_file>",inp[0]);return Ok(())}    
    let inpfile = &inp[1];
    let outfile = &inp[2];

    let open = loadtopo(File::open(inpfile).expect("Failed to read file"))?;
    let catfile = to_cartesian(open);
    let savepoint = saveashtml2(outfile, catfile);
    Ok(())
}
