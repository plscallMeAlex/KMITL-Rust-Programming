use rand::Rng;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
pub struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

pub fn gen_layer_list<R: Rng>(rng: &mut R, n:i64) -> Vec<Layer> {
    let mut res2 = Vec::new();
    for i in 1..=n{
        let mut res =Vec::new();
        let r = rng.gen_range(0..=255);let g = rng.gen_range(0..=255);let b = rng.gen_range(0..=255);let a = rng.gen_range(0..=255);
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
        let x = rng.gen_range(0..=500);let y = rng.gen_range(0..=500);
        let point = Point{x, y};
        res.push(point);
        res2.push(Layer{name:format!("Layer  {i}"), color:color.to_string(), points:res});
    }
    res2
}

pub fn gen_c2(write: &str, li:Vec<Layer>) -> io::Result<()> {
    let mut outfile = File::create(write)?;
    writeln!(outfile, r#"<svg width="500" height="500" xmlns="http://www.w3.org/2000/svg">"#)?;
    writeln!(outfile, r#"<rect width="100%" height="100%" fill="{}" />"#, "#FFFFFF")?;
    for i in &li {
        let point = &i.points;
        for j in point {
            writeln!(
                outfile,
                r#"    <circle cx="{}" cy="{}" r="20" fill="{}" />"#,
                j.x, j.y, i.color
            )?;
        }
    }
    writeln!(outfile, r#"</svg>"#)?;
    Ok(())
}