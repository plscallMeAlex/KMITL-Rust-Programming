use csv::{ReaderBuilder, WriterBuilder, Writer, Trim};
use std::io::{Read, Write};

#[derive(Debug, PartialEq)]
pub struct Point {
    x:f64,
    y:f64,
    color:String,
}

// impl Point {
//     pub fn new(x: f64, y:f64) -> Self {
//         Point{
//             x,
//             y,
//             color: String::new(),
//         }
//     }

// }
pub fn tag_points(pt_list:  Vec<Point>) -> Vec<Point> {
    let mut res = Vec::new();
    for i in pt_list {
        let inc = (i.x * i.x + i.y * i.y).sqrt();
        if inc <= 1.0 {
            res.push(Point{x: i.x, y: i.y, color: String::from("#80FF8080")});
        } else {
            res.push(Point{x: i.x, y: i.y, color: String::from("#FF808080")});
        }
    }
    res
}

#[test]
fn testq3_1() {
    let inp = vec![Point::new(0.,0.), Point::new(1.,1.), Point::new(0.2, 0.4)];
    let res = vec![Point { x: 0.0, y: 0.0, color: String::from("#80FF8080")}, Point { x: 1.0, y: 1.0, color: String::from("#FF808080") }, Point { x: 0.2, y: 0.4, color: String::from("#80FF8080")}];
    assert_eq!(tag_points(inp), res);
}

fn load_points<R: Read>(reader: R) -> Vec<(f64, f64, String)> {
    let mut rdr = ReaderBuilder::new()
                    .delimiter(b',')
                    .has_headers(true)
                    .trim(Trim::All)
                    .from_reader(reader);
    let mut out_lit = Vec::new();
    for i in rdr.records() {
        println!("{:?}",i);
        if let Ok(rec) = i {
            let x: f64 = rec[0].parse().unwrap();
            let y: f64 = rec[1].parse().unwrap();
            let color: String = rec[2].parse().unwrap();
            if color == "#000000" {
                out_lit.push((x, y, color))
            }else{continue}
        }
    }
    out_lit
}

#[test]
fn testq3_2() {
    let inp = "x, y, color\n1.0, 2.0, #000000\n0.1, 0.3, #011022\n".as_bytes();
    assert_eq!(load_points(inp), /*vec!["1.0", "2.0", "#000000"]*/ vec![(1.0, 2.0, "#000000".to_string())]);
}


fn save_points<W: std::io::Write>(writer: W, pt_list: Vec<Point>) {
    let mut wtr = Writer::from_writer(writer);
    for pt in pt_list {
        wtr.write_record(&[pt.x.to_string(), pt.y.to_string(), pt.color])
            .unwrap();
    }
    wtr.flush().unwrap();
}

#[test]
fn testq3_3() {
    let pt_list = vec![
        Point {
            x: 0.0,
            y: 0.0,
            color: String::new(),
        },
        Point {
            x: 0.5,
            y: 0.5,
            color: String::new(),
        },
        Point {
            x: 1.0,
            y: 1.0,
            color: String::new(),
        },
    ];
    let mut result = Vec::new();
    save_points(&mut result, pt_list);
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "0,0,\n0.5,0.5,\n1,1,\n");
}