fn main(){
    let args: Vec<String> = std::env::args().collect();
    let c_arg = if args.len() < 2 { "" } else { &args[1] };
    println!("Hello, {}",c_arg);
}
