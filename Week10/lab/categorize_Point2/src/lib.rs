use std::{vec, error::Error, io::Write, fs::File, process::Output};
use rand::Rng;

pub struct RandConfig {
    x_min:f64,
    x_max:f64,
    y_min:f64,
    y_max:f64,
}
pub struct Bound{
    circle1:Circle,
    circle2:Circle,
}

pub struct Circle {
    center:Point,
    radius:f64,
}

pub struct Point{ 
    x:f64,
    y:f64,
}

pub enum Checking{
    First(Point),
    Second(Point),
    Both(Point),
    Outrange(Point),
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

pub fn locate_n_point2(c:Bound, pt_list: Vec<Point>) -> Vec<Checking> {
    let mut res = Vec::new();
    for i in pt_list {
        let cal1 = (i.x-c.circle1.center.x).abs().powf(2.0) + (i.y-c.circle1.center.y).abs().powf(2.0);
        let cal2 = (i.x-c.circle2.center.x).abs().powf(2.0) + (i.y-c.circle2.center.y).abs().powf(2.0);
        if cal1.sqrt() <= c.circle1.radius && cal1.sqrt() <= c.circle2.radius {
            res.push(Checking::Both((i)));
        } else if cal1.sqrt() <= c.circle1.radius {
            res.push(Checking::First((i)));
        } else if cal2.sqrt() <= c.circle2.radius {
            res.push(Checking::Second(i));
        } else {
            res.push(Checking::Outrange(i));
        }
    }
    res
}

pub fn savehere(inp:Vec<f64>) {
    let mut rng = rand::thread_rng();
    
    let cfg = RandConfig {
        x_min: inp[1],
        x_max: inp[2],
        y_min: inp[3],
        y_max: inp[4],
    };

    let c1 = Circle {
        center: Point { x: inp[6], y: inp[7] },
        radius: inp[8],
    };
    
    let c2 = Circle {
        center: Point { x: inp[9], y: inp[10]},
        radius: inp[11],
    };

    let b = Bound{circle1:c1, circle2:c2};

    let pt_list = gen_point_list(rng, cfg, inp[5] as i64).unwrap();
    let loc_list = locate_n_point2(b, pt_list);
    
    let (w, h) = (600, 600); // SVG image size
    let mut file = File::create("dfdf.svg").unwrap();
    writeln!(file, r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, w, h).unwrap();
    writeln!(file, r#"<rect width="100%" height="100%" fill="{}" />"#, "#FFFFFF").unwrap();
    writeln!(file, r#"    <circle cx="{}" cy="{}" r="{}" fill="black" />"#, &inp[6], &inp[7], &inp[8]).unwrap();    
    writeln!(file, r#"    <circle cx="{}" cy="{}" r="{}" fill="yellow" />"#, &inp[9], &inp[10], &inp[11]).unwrap();
    for check in loc_list{
        match check {
            Checking::First(point) => {writeln!(file, r#"    <circle cx="{}" cy="{}" r="3" fill="green" />"#, point.x, point.y).unwrap();},
            Checking::Second(point) => {writeln!(file, r#"    <circle cx="{}" cy="{}" r="3" fill="red" />"#, point.x, point.y).unwrap();},
            Checking::Both(point) => {writeln!(file, r#"    <circle cx="{}" cy="{}" r="3" fill="magenta" />"#, point.x, point.y).unwrap();},
            Checking::Outrange(point) => {writeln!(file, r#"    <circle cx="{}" cy="{}" r="3" fill="blue" />"#, point.x, point.y).unwrap();},
        }
    }

    writeln!(file, r#"</svg>"#).unwrap();
}
// sfn locate_n_point2(b: Bound, pt_list: Vec<Point>)