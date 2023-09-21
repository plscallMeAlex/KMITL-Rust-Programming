//Problem 1.1
fn fahr_to_cel_1(f: &[f64]) -> Vec<f64>{
    let mut f_new = Vec::with_capacity(f.len()); //create new vec with the same size as array f to store a celsius
    for i in 0..f.len(){ //loop until all of value in array are converted
        let mut _c = (5./9.)*(f[i] - 32.);
        f_new.push(_c);//push a converted value to new vector
    }
    f_new //return new vector that contain celcius inside
}

fn main() {
    let f = [68., 14., 32., 50.]; //fix value
    let result = fahr_to_cel_1(&f);

    println!("fah: {:?}", f);
    println!("cel: {:?}", result);
}

#[test]
fn test_converting() {
    assert_eq!(fahr_to_cel_1(&[]), []);
    assert_eq!(fahr_to_cel_1(&[68., 14., 32., 50.]), [20.0, -10.0, 0.0, 10.0]);
}