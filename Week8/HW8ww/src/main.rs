fn main() {
    let points = vec![Point { x: 3, y: 4 }, Point { x: -1, y: -1 }];
    let polar_points = to_polar(points.clone());
    println!("Polar: {:?}", polar_points);

    let cartesian_points = to_cartesian(polar_points);
    println!("Cartesian: {:?}", cartesian_points);
}

use std::f64::consts::PI;

#[derive(Debug)]
pub struct Point {
    // Cartesian
    x: i64,
    y: i64,
}

#[derive(Debug)]
pub struct PolarPoint {
    // Polar
    r: i64,
    t: f64,
}

fn tan(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

pub fn to_polar(pt_list: Vec<Point>) -> Vec<PolarPoint> {
    let mut res = Vec::new();
    for i in pt_list {
        let r = ((i.x.pow(2) + i.y.pow(2)) as f64).sqrt();
        let t = tan(i.y as f64, i.x as f64);
        res.push(PolarPoint { r: r as i64, t });
    }
    res
}

pub fn to_cartesian(pt_list: Vec<PolarPoint>) -> Vec<Point> {
    let mut res = Vec::new();
    for i in pt_list {
        let nx = (i.r as f64 * i.t.cos()).round() as i64;
        let ny = (i.r as f64 * i.t.sin()).round() as i64;
        res.push(Point { x: nx, y: ny });
    }
    res
}

