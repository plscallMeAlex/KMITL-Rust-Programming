    // fn main() {
    //     let args: Vec<String> = std::env::args().collect();
    //     let f_arg = if args.len() < 2 { "" } else { &args[1] };
    //     let f:f32 = f_arg.parse().unwrap_or(0.0);
    //     println!("Celsius:{}", (5.0/9.0)*(f-32.0));
    // }

    // fn ctof(){
    //     let c: Vec<String> = std::env::args().collect();
    //     let c_arg = if c.len() < 2 { "" } else { &c[1] };
    //     let c:f32 = c_arg.parse().unwrap_or(0.0);
    //     print!("Farenheit:{}", ((9.0/5.0)*c)+32.0);
    // }

// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() > 1 {
//         let arg = &args[1];
//         if arg == "ctof" {
//             // Convert Celsius to Fahrenheit
//             ctof();
//         } else if arg == "ftoc"{
//             // Convert Fahrenheit to Celsius
//             let f: f32 = arg.parse().unwrap_or(0.0);
//             println!("Celsius: {}", (5.0 / 9.0) * (f - 32.0));
//         } else {
//             println!("Usage: cargo run <temperature> OR cargo run ctof");
//         }
//     }else {
//         println!("Usage: cargo run <temperature> OR cargo run ctof");
//     }
// }
    
// fn ctof() {
//     let c: Vec<String> = env::args().collect();
//     let c_arg: &str = if c.len() < 3 { "" } else { &c[2] };
//     let c: f32 = c_arg.parse().unwrap_or(0.0);
//     println!("Fahrenheit: {}", ((9.0 / 5.0) * c) + 32.0);
// }

use clap::{App,Arg};

fn main() {
    let matches = App::new("Temp")
        .version("1.1.1")
        .author("Alex")
        .about("Converting from Celsius to Farenheit or Farenheit to Celsius")
        .arg(
            Arg::with_name("Far")
                .valuename("farenheit")
                .required(false)
                .short("f")
                .validator(valid_nb)
        )
        .arc(
            Arg::with_name("Cel")
                .valuename("celsius")
                .required(false)
                .short("c")
                .validator(valid_nb)
        ).get_matches();
    
    let far = matches.value_of("Far").unwrap_or("0.0").to_string().parse()
        .unwrap();
    let cel = matches.value_of("Cel").unwrap_or("0.0").to_string().parse()
        .unwrap();

    if matches.is_present("far"){
        let tem_cel = (5.0 / 9.0) * (far - 32.0);
        println!("The temperature is {tem_cel} degrees celsius")
    }

    fn valid_nb() {
        match value.parse {
            
        }
    }
}