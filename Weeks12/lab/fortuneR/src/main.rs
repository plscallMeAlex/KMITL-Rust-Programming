fn main() {
    if let Err(e) = fortuneR::get_args().and_then(fortuneR::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}