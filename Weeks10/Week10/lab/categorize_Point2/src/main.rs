use std::error::Error;

use categorize_Point2::savehere;


fn main() -> Result<(), Box<(dyn Error)>>{
    let inp:Vec<f64> = std::env::args().map(|x| x.parse::<f64>().unwrap_or(0.0)).collect();
    // if inp.len() < 9{eprintln!("wrong format!");eprintln!("cargo run {{x_min}} {{x_max}} {{y_min}} {{y_max}} {{N_point}} {{Big_C_x}} {{Big_C_y}} {{Big_C_radius}}"); return Ok(());}
    let running = savehere(inp);
    Ok(())
}