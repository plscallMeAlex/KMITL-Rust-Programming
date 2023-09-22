use std::error::Error;
use std::io::Write;
use csv::Writer;

use rand::Rng;
#[derive(Debug)]
pub struct Point {
    x:i64,
    y:i64,
}
#[derive(Debug)]
pub struct Layer {
    name:String,
    color:String,
    points:Vec<Point>,
}

pub fn gen_layer_list<R: Rng>(rng: &mut R, n:i64) -> Vec<Layer> {
    let mut res2 = Vec::new();
    let round = rng.gen_range(20..=50);
    for i in 1..=n{
        let mut res =Vec::new();
        let r = rng.gen_range(0..=255);let g = rng.gen_range(0..=255);let b = rng.gen_range(0..=255);let a = rng.gen_range(0..=255);
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
        for j in 0..round {    
            let x = rng.gen_range(-100..=100);let y = rng.gen_range(-100..=100);
            let point = Point{x, y};
            res.push(point);
        }
        res2.push(Layer{name:format!("Layer  {i}"), color:color.to_string(), points:res});
    }
    res2
}

pub fn saving<W: std::io::Write>(write: W, pt_list: Vec<Layer>) -> Result<(), Box<dyn Error>> {
    let mut wrt = Writer::from_writer(write);
    for i in pt_list {
        let point = i.points;
        let mut cart = Vec::new();
        for j in point {
            cart.push(j.x.to_string());
            cart.push(j.y.to_string());
        }
        wrt.write_record(&[i.name.to_string(), i.color.to_string(), cart.join(",")])?;
    }
    wrt.flush()?;    
    Ok(())
}
