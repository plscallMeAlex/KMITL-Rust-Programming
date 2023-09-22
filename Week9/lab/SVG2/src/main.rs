use rand::Rng;
use SVG2::{gen_layer_list,gen_c2};
fn main() {
    let mut rng = rand::thread_rng();
    let fst =gen_layer_list(&mut rng, 5);
    let snd = gen_c2("Hi2.svg", fst);
}
