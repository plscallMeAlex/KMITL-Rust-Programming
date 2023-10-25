use std::{f64::consts::PI, fmt::format, char, str::Chars};

//Question-1.2

trait Shape {
    fn rep_string(&self) -> String;
    fn area(&self) -> String;
    fn clone(&self) -> Box<dyn Shape>;
}

#[derive(Clone)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

#[derive(Clone)]
struct Rectangle {
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

#[derive(Clone)]
struct Triangle{
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64)
}

impl Circle {
    fn new(x: i64, y: i64, r: i64) -> Box<dyn Shape> {
        return Box::new(Circle { x, y, r });
    }
}

impl Rectangle {
    fn new(x: i64, y: i64, w: i64, h: i64) -> Box<dyn Shape> {
        return Box::new(Rectangle { x, y, w, h });
    }
}

impl Triangle {
    fn new(x1:f64,y1:f64, x2:f64, y2:f64, x3:f64, y3:f64) -> Box<dyn Shape> {
        return Box::new(Triangle{p1:(x1,y1), p2:(x2,y2), p3:(x3,y3)})
    }

}

impl Shape for Circle {
    fn rep_string(&self) -> String {
        return format!("<Circle: {}, {}, {}>", self.x, self.y, self.r);
    }
    fn area(&self) -> String {
        let area = self.r as f64 * self.r as f64 * PI;
        return format!("{:.2}", area);
    }
    fn clone(&self) -> Box<dyn Shape> {
        Box::new(Clone::clone(self))
    }
}

impl Shape for Rectangle {
    fn rep_string(&self) -> String {
        return format!(
            "<Rectangle: {}, {}, {}, {}>",
            self.x, self.y, self.w, self.h
        );
    }
    fn area(&self) -> String {
        let area = self.w as f64 * self.h as f64;
        return format!("{:.2}", area);
    }
    fn clone(&self) -> Box<dyn Shape> {
        Box::new(Clone::clone(self))
    }
}

impl Shape for Triangle {
    fn rep_string(&self) -> String {
        return format!(
            "<Triangle: p1{:?}, p2{:?}, p3{:?}>",
            self.p1, self.p2, self.p3
        );
    }
    fn area(&self) -> String {
        let are = 0.5 * ((self.p1.0 - self.p3.0)*(self.p2.1 - self.p1.1) - (self.p1.0 - self.p2.0)*(self.p3.1 - self.p1.1));
        return  format!("{:.2}",are);
    }
    fn clone(&self) -> Box<dyn Shape> {
        Box::new(Clone::clone(self))
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        Shape::clone(self.as_ref())
    }
}

fn main() {
    println!("PP")
}
