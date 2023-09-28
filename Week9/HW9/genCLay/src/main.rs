use std::error::Error;
use rand::Rng;
use genCLay::{gen_obj_layer_list, savefile, loadfile};
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let inp:Vec<String> = std::env::args().collect();
    if inp.len() != 3 {eprintln!("Enter wrong format!: cargo run [layer] [filename]"); return Ok(())}
    let n = &inp[1].parse::<i64>().unwrap();
    let outfile = format!("{}.csv", &inp[2]);
    let mut rng = rand::thread_rng();
    let frt = gen_obj_layer_list(rng, *n);
    let create = File::create(outfile).unwrap();
    savefile(create, frt);
    Ok(())
}
