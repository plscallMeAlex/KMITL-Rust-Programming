//Problem 1.2
fn fahr_to_cel_2(f: &[f64]) -> Vec<f64>{
    let mut f_new = Vec::with_capacity(f.len());
    


    fn ftoc(f: &[f64], f_new: &mut Vec<f64>, round: usize){ //use round as a counter of this recursion 
        if round < f.len() { //it will call function until this statement is false
            f_new.push((5./9.)*(f[round]-32.)); //store a converted value in new vec
            ftoc(f, f_new, round +1); //call function 
        }
    }
    ftoc(f, &mut f_new, 0);//star function with counter = 0 
    f_new
}

fn main() {
    let f = [68., 14., 32., 50.];
    let result = fahr_to_cel_2(&f);

    println!("fah: {:?}", f);
    println!("cel: {:?}", result);
}

#[test]
fn test_converting() {
    assert_eq!(fahr_to_cel_2(&[]), []);
    assert_eq!(fahr_to_cel_2(&[68., 14., 32., 50.]), [20., -10., 0., 10.]);
}