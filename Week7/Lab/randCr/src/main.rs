use rand::Rng;

fn gen_number<R: Rng>(rng: &mut R) -> f64 {
    rng.gen_range(-1.0..=1.0)
}

//Q3.3
// fn main() {
//     let arg: Vec<String> = std::env::args().collect();
//     let n_arg = if arg.len() < 2 {""} else {&arg[1]};
//     let n:i64 = n_arg.parse().expect("Input Sth.");
//     let mut rng = rand::thread_rng();
//     let mut count = 0;
//     let mut numm =  Vec::new();
//     if n <= 0 { 
//         println!("0.0");
//     } else {
//         loop{
//             let num1 = gen_number(&mut rng);        
//             if num1 >= -1.0 && num1 <= 1.0 {
//                 numm.push(num1);
//             }
//             count += 1;
//             if count == n {
//                 break;
//             }
//         }
//         let prob = numm.len() as f64 / count as f64;
//         println!("Prob: {:.2}", prob)
//     }
// }

fn main() {
    let arg: Vec<String> = std::env::args().collect();
    let n_arg = if arg.len() < 2 {""} else {&arg[1]};
    let n:i64 = n_arg.parse().expect("Input Sth.");
    let mut rng = rand::thread_rng();
    let mut count = 0;
    let mut tuu:Vec<(f64,f64)> = Vec::new();
    if n <= 0 {
        println!("0.0");
    } else {
        loop{
            let x = gen_number(&mut rng);
            let y = gen_number(&mut rng);
            if (x * x) + (y * y) <= 1. {
                tuu.push((x, y));
            }
            count += 1;
            if count == n {
                break;
            }
        }
        let prob = (tuu.len() as f64 / count as f64) * 4.0;
        println!("Prob: {prob}");
    }

}