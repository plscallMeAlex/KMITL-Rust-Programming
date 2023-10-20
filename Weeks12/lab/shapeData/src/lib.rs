use std::{f64::consts::PI, fmt::format, hash, char, str::Chars};

//------------------------------------------------------------------------------------------------------------------------

// #[derive(Debug)]
// enum Shape {
//     Circle(i64, i64, i64),
//     Rectangle(i64, i64, i64, i64),
// }

// trait Shapefn {
//     fn rep_string(&self) -> String;
//     fn area(&self) -> String;
// }

// impl Shapefn for Shape {
//     fn rep_string(&self) -> String {
//         match self {
//             Self::Circle(x, y , r) => return format!("Circle: {}, {}, {}",x,y,r),
//             Self::Rectangle(x, y, w, h) => return  format!("Rectangle: {}, {}, {}, {}", x, y, w, h)
//         }
//     }
//     fn area(&self) -> String {
//         match self {
//             Self::Circle(x, y , r) => {
//                 let are = PI * (*r as f64) * (*r as f64);
//                 return format!("area: {:.2}",are)},
//             Self::Rectangle(x, y, w, h) => {
//                 let are = w * h;
//                 return  format!("area: {:.2}", are)}
//         }
//     }
// }

// // ------------------------------------------------------------------------------------------------------------------------

// const INPUT_SHAPES: &[Shape] = &[
//     Shape::Circle(0, 0, 1),
//     Shape::Circle(50, 50, 15),
//     Shape::Rectangle(40, 40, 20, 20),
//     Shape::Rectangle(10, 40, 15, 10),
// ];

// const EXPECTED: &[&str] = &[
//     "<Circle: 0, 0, 1>, area: 3.14",
//     "<Circle: 50, 50, 15>, area: 706.86",
//     "<Rectangle: 40, 40, 20, 20>, area: 400.00",
//     "<Rectangle: 10, 40, 15, 10>, area: 150.00",
// ];
// #[test]
// fn test_shapes() {
//     let input_list = INPUT_SHAPES;
//     let shape_list = input_list.clone();
//     let omap = shape_list
//         .iter()
//         .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
//     let output: Vec<_> = omap.collect();
// }

// //------------------------------------------------------------------------------------------------------------------------

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

#[derive(Clone)]
struct Rectangle {
    x: i64,
    y: i64,
    w: i64,
    h: i64,
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

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        Shape::clone(self.as_ref())
    }
}

//------------------------------------------------------------------------------------------------------------------------

fn input_shape_list() -> Vec<Box<dyn Shape>> {
    vec![
        Circle::new(0, 0, 1),
        Circle::new(50, 50, 15),
        Rectangle::new(40, 40, 20, 20),
        Rectangle::new(10, 40, 15, 10),
    ]
}
const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>",
    "<Circle: 50, 50, 15>",
    "<Rectangle: 40, 40, 20, 20>",
    "<Rectangle: 10, 40, 15, 10>",
];
const EXPECTED_002: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
];

#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}

#[test]
fn test_shapes_002() {
    let shape_list = input_shape_list();
    let omap = shape_list
        .iter()
        .map(|s| format!("{}, area: {}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

#[test]
fn test_shapes_003() {
    let input_list = input_shape_list();
    let shape_list = input_list.clone();
    let omap = shape_list
        .iter()
        .map(|s| format!("{}, area: {}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

//------------------------------------------------------------------------------------------------------------------------

// #[derive(Clone)]
// enum Text {
//     Plain(String),
//     Repeated(Box<Text>,usize),
// }

// impl Text {
//     fn value(&self) -> String {
//         match self {
//             Text::Plain(t) => t.clone(),
//             Text::Repeated(t, n) => t.value().repeat(*n),
//         }
//     }
// }

// impl From<&Text> for Box<Text> {
//     fn from(t: &Text) -> Box<Text> {
//         match t {
//             Text::Plain(c) => Box::new(Text::Plain(c.clone())),
//             Text::Repeated(c, n) => Box::new(Text::Repeated(c.clone(), *n))
//         }
//     }
// }

// impl AsRef<Text> for Text {
//     fn as_ref(&self) -> &Text {
//         &self
//     }
// }

// #[test]
// fn test_text_repeated() {
//     let t1 = Text::Plain("Hi".into());
//     let t2 = Text::Plain("[+]".into());
//     let t3 = Text::Repeated(t2.as_ref().into(), 3);
//     let t4 = Text::Repeated(t3.as_ref().into(), 5);
//     assert_eq!(t1.value(), "Hi");
//     assert_eq!(t2.value(), "[+]");
//     assert_eq!(t3.value(), "[+]".repeat(3));
//     assert_eq!(t4.value(), "[+]".repeat(15));
// }

//------------------------------------------------------------------------------------------------------------------------

// trait Text {
//     fn value(&self) -> String;
//     fn clone_box(&self) -> Box<dyn Text>;
// }

// impl Clone for Box<dyn Text> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

// #[derive(Clone)]
// struct PlainText {
//     chars: String,
// }

// struct RepeatedText {
//     chars: Box<dyn Text>,
//     round: usize,
// }

// impl From<&str> for PlainText {
//     fn from(text: &str) -> PlainText {
//         PlainText {
//             chars: text.to_string(),
//         }
//     }
// }

// impl Text for PlainText {
//     fn value(&self) -> String {
//         self.chars.clone()
//     }
//     fn clone_box(&self) -> Box<dyn Text> {
//         Box::new(self.clone())
//     }
// }

// impl Text for RepeatedText {
//     fn clone_box(&self) -> Box<dyn Text> {
//         Box::new(RepeatedText{chars: self.chars.clone_box(), round:self.round})
//     }
//     fn value(&self) -> String {
//         self.chars.value().repeat(self.round)
//     }
// }

// impl AsRef<dyn Text> for PlainText {
//     fn as_ref(&self) -> &(dyn Text + 'static) {
//         self
//     }
// }


// impl RepeatedText {
//     fn with_parts(charss:&dyn Text, num:usize) -> RepeatedText {
//         RepeatedText { chars: charss.clone_box(), round: num }
//     }
// }

// #[test]
// fn test_text_repeated() {
//     let t1 = PlainText::from("Hi");
//     let t2 = PlainText::from("[+]");
//     let t3 = RepeatedText::with_parts(&t2, 3);
//     let t4 = RepeatedText::with_parts(&t3, 5);
//     assert_eq!(t1.value(), "Hi");
//     assert_eq!(t2.value(), "[+]");
//     assert_eq!(t3.value(), "[+]".repeat(3));
//     assert_eq!(t4.value(), "[+]".repeat(15));
// }
