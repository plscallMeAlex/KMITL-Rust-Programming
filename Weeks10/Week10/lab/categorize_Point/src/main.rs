use std::error::Error;

use categorize_Point::{gen_point_list, locate_n_point, Circle, Point, RandConfig, savehere};
use rand::Rng;
fn main() -> Result<(), Box<dyn Error>>{
    let inp:Vec<f64> = std::env::args().map(|x| x.parse::<f64>().unwrap_or(0.0)).collect();
    // if inp.len() < 9{eprintln!("wrong format!");eprintln!("cargo run {{x_min}} {{x_max}} {{y_min}} {{y_max}} {{N_point}} {{Big_C_x}} {{Big_C_y}} {{Big_C_radius}}"); return Ok(());}
    let running = savehere(inp);
    Ok(())
    // let mut rng = rand::thread_rng();
    // let cfg = RandConfig {
    //     x_min: -1.5,
    //     x_max: 1.5,
    //     y_min: -1.5,
    //     y_max: 1.5,
    // };
    // let c = Circle {
    //     center: Point { x: -0.1, y: -0.1 },
    //     radius: 0.8,
    // };
    // let pt_list = gen_point_list(rng, cfg, 50).unwrap();
    // let loc_list = locate_n_point(c, pt_list);
    // for loc in &loc_list {
    //     println!("{loc:?}");
    // }
    // save_to_svg(loc_list, "Hi.svg").unwrap();
    //just 3.3 q to make function to save output file

   
   
    // let (w, h) = (600, 600); // SVG image size
    // let scale = (h as f64) / (cfg.y_max - cfg.y_min);
    // let pt_map = |pt: &Point| {
    //     let x = (pt.x - cfg.x_min) * scale;
    //     let y = (-pt.y - cfg.y_min) * scale;
    //     (x as i64, y as i64) // map (x, y) to SVG output
    // };
}

// fn main() -> Result<(), Box<dyn Error>> {
// ]
//     let points = gen_point_list(rand::thread_rng(), cfg, 10)?;
//     let checks = locate_n_point(Circle { center: Point { x: 0.0, y: 0.0 }, radius: 0.5 }, points);

//     save_to_svg(checks, "output.svg")?;

//     Ok(())
// }    


// }
