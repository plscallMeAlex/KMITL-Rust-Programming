use std::error::Error;
use rand::Rng;
use generateLayer::{gen_layer_list, saving};
use std::{env, fs::File};

fn main() {
    let inp:Vec<String> = std::env::args().collect();
    if inp.len() < 3{
        return;
    }
    let mut rng = rand::thread_rng();
    let numlayer:i64 = inp[1].parse().unwrap();
    let outfile = &inp[2];
    let fst = gen_layer_list(&mut rng, numlayer);

    let creating = File::create(outfile).unwrap();
    let _ = saving(creating, fst);
}