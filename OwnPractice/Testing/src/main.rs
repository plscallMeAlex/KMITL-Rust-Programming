// fn slope(x1:i64, y1:i64, x2:i64, y2:i64) -> f64 {
//     let sp:f64 = (y2 as f64 -y1 as f64)/(x2 as f64 -x1 as f64);
//     sp
// }

// fn main() {
//     let co:Vec<i64> = std::env::args().parse().unwrap().collect();
//     let m: f64 = slope(co[1], co[2], co[3], co[4]);
//     println!("{}", m);
// }

// fn main() {
//     let input:Vec<i32> = std::env::args().parse().collect();

// }

// fn triangle_patterns(inp:i32) {
//     if inp < 0 {
//         println!("Input Some num");
//     } else {
//         for eline in inp {
//             for estar in eline {
//                 print!("*");
//             }
//             println!();
//         }
//     }
// }

use std::result;

use Testing::{triangle_patterns, count_negative, doubles, quote_list, fahr_to_cel_v, reverse, quote_list_r, arrow_right, count_vowels_r};

fn main() {
    // triangle_patterns();
    // let inp:Vec<String> = std::env::args().collect();
    // if inp.len() != 2 {
    //     println!("sdd");
    //     return;
    // }
    // let result = count_negative(in.into_boxed_slice());
    // println!("{}",result)
    // let inp:[i64;5] = [6, 7, 7, 2, 1];
    // let x =count_negative(&[1, 2, -3, 4, -6, 7]);
//     let x = doubles(&inp);
//     let y = quote_list(&[""; 0], '*');
//     let z = quote_list(&["abcd", "xyz"], '*');
//     println!("{:?}", &x);
//     println!("{:?}", &y);
//     println!("{:?}", &z);
//     let ftoc = fahr_to_cel_v(&[6, -3, 77, 35]);
//     println!("{:?}", ftoc);
    // let x:[i64;6] = [1, 2, 3, 4, 5, 6];
        // let x1 = reverse(&x);
        // println!("{:?}",&x1);
        // println!("{:?}", doubles_r(&[4,5,6]));
    // println!("{:?}", quote_list_r(&["JAva", "ddd", "Rust"], '*'));
    // println!("{}", arrow_right(5));
    println!("{}", count_vowels_r("aeiottttsu"));
}


