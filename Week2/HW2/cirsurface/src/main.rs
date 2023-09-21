// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     let c_arg = if args.len() < 2 { "" } else { &args[1] };
//     let c: f32 = c_arg.parse().unwrap_or(0.0);
//     println!("your circle area is: {}", c*c*3.1416);
// }

use clap::{Arg, App};

fn main() {
    let matches = App::new("cc_area")
                  .author("Alex")
                  .version("1.0.2")
                  .about("Collecting a radius for find a circle area")
                  .arg(
                    Arg::with_name("rad")
                        .value_name("Radius")
                        .help("Enter a number for radius")
                        .required(true)
                        .short("r")
                        .index(1)
                   ).get_matches();

    let radius = matches.value_of("rad").unwrap().to_string().parse().unwrap_or(0.0);
    let pi = 3.14159;
    let c_ar = pi * radius * radius;

    println!("The Area of the circle is: {c_ar}")
}
