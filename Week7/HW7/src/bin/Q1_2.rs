fn main() {
    let mut inp:Vec<String> = std::env::args().collect();
    if inp.len() < 2 {return}
    let inp = &inp[1..];
    let mut rinp: Vec<i64> = Vec::new();
    for i in inp {
        rinp.push(i.parse::<i64>().unwrap());
    }
    
    for i in 0..rinp.len() {
        for j in 0..rinp.len() - 1 {
            if rinp[j] > rinp[j + 1] {
                rinp.swap(j, j + 1);
            }
        }
    }
    println!("{:?}", rinp);
    
    for i in 0..rinp.len() {
        for j in 0..rinp.len() - 1 {
            if rinp[j] < rinp[j + 1] {
                rinp.swap(j, j + 1);
            }
        }
    }
    println!("{:?}", rinp);
}