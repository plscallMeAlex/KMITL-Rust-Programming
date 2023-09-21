fn main() {
    let inp:Vec<f64> = std::env::args().filter_map(|arg| arg.parse::<f64>().ok()).collect();
    let mut point:Vec<(f64, f64)> = Vec::new();
    for i in 0..inp.len() {
        if i % 2 == 0 {
            if i == inp.len() - 1 {break;}
            point.push((inp[i], inp[i+1]))
        }
    }
    point.sort_by(|(a, b), (c, d)| {
        a.partial_cmp(c)
            .unwrap()
            .then_with(|| b.partial_cmp(d).unwrap())
    });
    println!("Asc: {:?}", point);
    point.sort_by(|(a, b), (c, d)| {
        c.partial_cmp(a)
            .unwrap()
            .then_with(|| d.partial_cmp(b).unwrap())
    });

    println!("Des: {:?}", point);
}