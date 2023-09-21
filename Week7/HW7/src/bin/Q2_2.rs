fn main() {
    let inp:Vec<f64> = std::env::args().filter_map(|arg| arg.parse::<f64>().ok()).collect();
    let mut point:Vec<(f64, f64)> = Vec::new();
    if inp.is_empty() {
        return;
    } else {
        for i in 0..inp.len() {
            if i % 2 == 0 {
                if i == inp.len() - 1 {break;}
                point.push((inp[i], inp[i+1]))
            }
        }
        //asc
        for i in 0..point.len()-1 {
            for j in 0..point.len()-1 {
                if point[j].0 > point[j+1].0{
                    point.swap(j, j+1);
                } 
            }
            for k in 0..point.len()-1 {
                if point[k].0 == point[k+1].0 && point[k].1 > point[k+1].1{
                    point.swap(k, k+1);
                }
            }
        }
        // for i in 0..point.len()-1 {
        //     if 
        // }

        println!("Asc: {:?}",point);
        //des
        for i in 0..point.len()-1 {
            for j in 0..point.len()-1 {
                if point[j].0 < point[j+1].0 {
                    point.swap(j, j+1);
                }
            }
            for k in 0..point.len()-1 {
                if point[k].0 == point[k+1].0 && point[k].1 < point[k+1].1{
                    point.swap(k, k+1);
                }
            }
        }
        println!("Des: {:?}",point);
    }
}