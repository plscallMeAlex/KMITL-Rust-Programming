use std::env;

fn slope(x1:f64, y1:f64, x2:f64, y2:f64) {
    let m:f64 = (y2 - y1)/(x2 - x1);
    println!("Slope is: {}",m);
}   

fn main() {
    let point: Vec<String> = env::args().collect();
    let x1:&f64 = &point[1].parse().unwrap_or(0.0);
    let x2:&f64 = &point[3].parse().unwrap_or(0.0);
    let y1:&f64 = &point[2].parse().unwrap_or(0.0);
    let y2:&f64 = &point[4].parse().unwrap_or(0.0);
    if point.len() == 5 {
        slope(*x1,*y1,*x2,*y2);
    }else if point.len() < 5 {
        println!("Input more");
    }else {
        println!("Input")
    }
}


