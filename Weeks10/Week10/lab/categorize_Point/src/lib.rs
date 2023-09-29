use std::{vec, error::Error, io::Write, fs::File, process::Output};

use rand::Rng;

#[derive(Debug)]
pub struct RandConfig {
    pub x_min:f64,
    pub x_max:f64,
    pub y_min:f64,
    pub y_max:f64,
}

#[derive(Debug)]
pub struct Point{
    pub x:f64,
    pub y:f64,
}

#[derive(Debug)]
pub struct Pointwithdis{
    pub point: Point,
    pub distance: f64,
}

#[derive(Debug)]
pub struct Circle{
    pub center:Point,
    pub radius:f64,
}

#[derive(Debug)]
pub enum Check {
    Inside(Point),
    Outside(Point),
}

pub fn gen_point_list<R: Rng>(mut rng: R, cfg:RandConfig, n:i64) -> Result<Vec<Point>, Box<dyn Error>> {
    if n < 0{eprintln!("nothing happen")}
    let mut result = Vec::new();
    for i in 0..n{
        let x = rng.gen_range(cfg.x_min..=cfg.x_max);
        let y = rng.gen_range(cfg.y_min..=cfg.y_max);
        result.push(Point{x:x, y:y});
    }
    Ok(result)
}

pub fn locate_n_point(c:Circle, pt_list: Vec<Point>) -> Vec<(Check, f64)>{
    let mut result = Vec::new();
    for i in pt_list {
        let cal = (i.x-c.center.x).abs().powf(2.0) + (i.y-c.center.y).abs().powf(2.0);
        let distance = format!("{:.2}",cal.sqrt()).parse::<f64>().unwrap();
        if cal.sqrt() < c.radius {
            result.push((Check::Inside(i), c.radius));
        } else {
            result.push((Check::Outside(i), c.radius));
        }
    }
    result
}


pub fn savehere(inp:Vec<f64>) {
    let mut rng = rand::thread_rng();
    
    let cfg = RandConfig {
        x_min: inp[1],
        x_max: inp[2],
        y_min: inp[3],
        y_max: inp[4],
    };

    let c = Circle {
        center: Point { x: inp[6], y: inp[7] },
        radius: inp[8],
    };

    let pt_list = gen_point_list(rng, cfg, inp[5] as i64).unwrap();
    let loc_list = locate_n_point(c, pt_list);
    
    let (w, h) = (600, 600); // SVG image size
    let mut file = File::create("dfdf.svg").unwrap();
    writeln!(file, r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, w, h).unwrap();
    writeln!(file, r#"<rect width="100%" height="100%" fill="{}" />"#, "#FFFFFF").unwrap();
    writeln!(file, r#"    <circle cx="{}" cy="{}" r="{}" fill="black" />"#, &inp[6], &inp[7], &inp[8]).unwrap();
    for (check, _) in loc_list{
        match check {
            Check::Inside(point) => {writeln!(file, r#"    <circle cx="{}" cy="{}" r="3" fill="green" />"#, point.x, point.y).unwrap();},
            Check::Outside(point) => {writeln!(file, r#"    <circle cx="{}" cy="{}" r="3" fill="red" />"#, point.x, point.y).unwrap();},
        }
    }

    writeln!(file, r#"</svg>"#).unwrap();
}
// pub fn save_to_svg(points: Vec<(Check, f64)>, filename: &str) -> Result<(), Box<dyn Error>> {
//     let (w, h) = (600, 600); // SVG image size
//     let mut file = File::create(filename)?;

//     writeln!(file, r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, w, h)?;
//     writeln!(file, r#"<rect width="100%" height="100%" fill="{}" />"#, "#FFFFFF")?;
//     writeln!(file, r#"    <circle cx="{}" cy="{}" r="{}" fill="black" />"#,)?;
//     for (check, rad) in points {
//         match check {
//             Check::Inside(point) => {
//                 let x = point.x;
//                 let y = point.y;
//                 let radius = rad;
//                 writeln!(file, r#"    <circle cx="{}" cy="{}" r="{}" fill="green" />"#, x, y, rad)?;
//             }
//             Check::Outside(point) => {
//                 let x = point.x;
//                 let y = point.y;
//                 let radius = rad;
//                 writeln!(file, r#"    <circle cx="{}" cy="{}" r="{}" fill="red" />"#, x, y, radius)?;
//             }
//         }
//     }

//     writeln!(file, r#"</svg>"#)?;
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     // Example usage
//     let cfg = RandConfig {
//         x_min: -1.0,
//         x_max: 1.0,
//         y_min: -1.0,
//         y_max: 1.0,
//     };

//     let points = gen_point_list(rand::thread_rng(), cfg, 10)?;
//     let checks = locate_n_point(Circle { center: Point { x: 0.0, y: 0.0 }, radius: 0.5 }, points);

//     save_to_svg(checks, "output.svg")?;

//     Ok(())
// }
// pub fn gen_c2(li:Vec<Check>) {
//     let mut outfile = File::create("Hi.svg").unwrap();
//     writeln!(outfile, r#"<svg width="600" height="600" xmlns="http://www.w3.org/2000/svg">"#);
//     writeln!(outfile, r#"<rect width="100%" height="100%" fill="{}" />"#, "#FFFFFF");
//     for i in &li {
//         let point = &i.;
//         for j in point {
//             writeln!(
//                 outfile,
//                 r#"    <circle cx="{}" cy="{}" r="20" fill="green" />"#,
//                 j.x, j.y, i.color
//             );
//         }
//     }
//     writeln!(outfile, r#"</svg>"#);
//     Ok(())
// }