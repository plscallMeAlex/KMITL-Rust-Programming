// use rand::Rng;
// use std::fs::File;
// use std::io::Write;


// #[derive(Debug, PartialEq)]
// pub struct Point {
//     x:i64,
//     y:i64,
// }

// #[derive(Debug)]
// pub struct Layer {
//     name:String,
//     color:String,
//     points:Vec<Point>,
// }

// pub fn gen_layer<R: Rng>(rng: &mut R, name:String, color:String) -> Layer { 
//     let round = rng.gen_range(20..=50);
//     let mut res = Vec::new();
//     for i in 0..round {
//         let x = rng.gen_range(0..=500);
//         let y = rng.gen_range(0..=500);
//         let point = Point{x, y};
//         res.push(point);        
//     }
//     Layer{
//         name:name,
//         color:color,
//         points:res,
//     }
// }

// pub fn genCircle(write: &str, l:Layer) {
//     let mut outfile = File::create(write);
//     writeln!(outfile, r#"<svg width="500" height="500" xmlns="http://www.w3.org/2000/svg">"#);
//     writeln!(outfile, r#"<    rect width="100%" height="100%" fill="{}" />"#,l.color);
//     for point in &l.points {
//         writeln!(
//             outfile,
//             r#"    <circle cx="{}" cy="{}" r="20" fill="{}" />"#,
//             point.x, point.y, l.color
//         )?;
//     }
//     writeln!(outfile, r#"</svg>"#)
// }


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

pub fn gen_layer<R: Rng>(rng: &mut R, name: String, color: String) -> Layer {
    let round = rng.gen_range(20..=50);
    let mut res = Vec::new();
    for _ in 0..round {
        let x = rng.gen_range(0..=500);
        let y = rng.gen_range(0..=500);
        let point = Point { x, y };
        res.push(point);
    }
    Layer {
        name,
        color,
        points: res,
    }
}

pub fn gen_circle(write: &str, l: Layer) -> io::Result<()> {
    let mut outfile = File::create(write)?;

    writeln!(outfile, r#"<svg width="500" height="500" xmlns="http://www.w3.org/2000/svg">"#)?;
    writeln!(outfile, r#"<rect width="100%" height="100%" fill="{}" />"#, "#FFFFFF")?;

    for point in &l.points {
        writeln!(
            outfile,
            r#"    <circle cx="{}" cy="{}" r="20" fill="{}" />"#,
            point.x, point.y, l.color
        )?;
    }

    writeln!(outfile, r#"</svg>"#)?;

    Ok(())
}
