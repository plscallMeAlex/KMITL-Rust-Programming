use rand::Rng;
use SVG1::{gen_layer,gen_circle};
fn main() {
    let mut rng = rand::thread_rng();
    let fst =gen_layer(&mut rng, "dfdf".to_string(), "#000000".to_string());
    let snd = gen_circle("Hi1.svg", fst);
}
