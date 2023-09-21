fn main() {
    let mut inp:Vec<String> = std::env::args().filter_map(|arg| arg.parse().ok()).collect(); 
    if inp.len() < 2 {return}
    let inp = &inp[1..];
    let mut rinp:Vec<i64> = Vec::new();
    for i in inp{
            rinp.push(i.parse::<i64>().unwrap())
    }
    rinp.sort_by(|a, b| a.cmp(b));
    println!("{:?}", rinp);
    rinp.sort_by(|a, b| b.cmp(a));
    println!("{:?}", rinp);
}