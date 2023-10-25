use std::{f64::consts::PI, fmt::format, char, str::Chars};
//Question-1.1

#[derive(Debug)]
enum Shape {
    Circle(i64, i64, i64),
    Rectangle(i64, i64, i64, i64),
    Triangle((f64,f64),(f64,f64),(f64,f64)),
}

trait Shapefn {
    fn rep_string(&self) -> String;
    fn area(&self) -> String;
}

impl Shapefn for Shape {
    fn rep_string(&self) -> String {
        match self {
            Self::Circle(x, y , r) => return format!("Circle: {}, {}, {}",x,y,r),
            Self::Rectangle(x, y, w, h) => return  format!("Rectangle: {}, {}, {}, {}", x, y, w, h),
            Self::Triangle(a, b, c) => return format!("Triangle: p1{:?}, p2{:?}, p3{:?}", a, b, c)
        }
    }
    fn area(&self) -> String {
        match self {
            Self::Circle(x, y , r) => {
                let are = PI * (*r as f64) * (*r as f64);
                return format!("area: {:.2}",are)},
            Self::Rectangle(x, y, w, h) => {
                let are = w * h;
                return  format!("area: {:.2}", are)},
            Self::Triangle(a, b, c) => {
                let are = 0.5 * ((a.0 - c.0)*(b.1 - a.1) - (a.0 - b.0)*(c.1 - a.1));
                return  format!("area: {:.2}", are)
            }
        }
    }
}

fn main() {
    println!("PP")
}
