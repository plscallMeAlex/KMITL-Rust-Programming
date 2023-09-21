use std::vec;

use rand::Rng;

//Q3.1
fn filter_number(nl: &[f64]) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in nl {
        match i {
            -1.0..=1.0 => result.push(*i),
            _ => continue,
        }
    }
    result
}

#[test]
fn test_filter_numbers() {
    assert_eq!(filter_number(&[]), ([]));
    assert_eq!(
        filter_number(&[2.0, 0.4, 5.0, -1.0, 0.8]),
        ([0.4, -1.0, 0.8])
    );
}

//Q3.2
fn gen_numbers<R: Rng>(rng: &mut R, n: i64) -> Vec<f64> {
    if n <= 0 {
        return Vec::new();
    }
    let mut res: Vec<f64> = Vec::new();
    loop {
        let randnum = rng.gen_range(-50.0..=50.0);
        if randnum >= -10.0 && randnum <= 10.0 {
            res.push(randnum);
        }
        if res.len() == n as usize {
            break;
        }
    }
    res
}

#[test]
fn test_gen_number() {
    let mut rng = rand::thread_rng();

    let nah = gen_numbers(&mut rng, 0);
    assert_eq!(nah.len(), 0);

    let plus = gen_numbers(&mut rng, 5);
    assert_eq!(plus.len(), 5);
    for &num in &plus {
        assert!(num >= -10.0 && num <= 10.0);
    }

    let minus = gen_numbers(&mut rng, -5);
    assert_eq!(minus.len(), 0);
}

//Q3.3
pub fn prob<R: Rng>(rng: &mut R, n:i64) -> f64 {
    let mut count = 0;
    let mut numm =  Vec::new();
    if n <= 0 { 
        return 0.0;
    }
    loop{
        let num1 = rng.gen_range(-10.0..=10.0);        
        if num1 >= -1.0 && num1 <= 1.0 {
            numm.push(num1);
            count += 1;
        }
        if count == n {
            break;
        }
    }
    let prob = numm.len() as f64 / n as f64;
    prob
}

//Q4.1
fn filter_points(pt_list: &[(f64, f64)]) -> Vec<(f64, f64)> {
    if pt_list.is_empty() {
        return Vec::new();
    }
    let mut res = Vec::new();
    for &(x, y) in pt_list.iter() {
        if x * x + y * y <= 1.0 {
            res.push((x, y));
        }
    }
    res
}

#[test]
fn test_filter_points() {
    assert_eq!(filter_points(&[]), Vec::new());
    assert_eq!(filter_points(&[(2.0, 3.0), (5.0, 6.0)]), Vec::new());
    assert_eq!(filter_points(&[(-2.0, -2.0), (-1.5, 1.5), (0.0, 0.0), (1.5, -1.5), (2.0, 2.0)]), vec![(0.0, 0.0)]);
    assert_eq!(
        filter_points(&[(-2.0, -2.0), (-1.5, -1.5), (-1.0, -1.0), (-0.5, -0.5), (0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (1.5, 1.5), (2.0, 2.0)]),
        vec![(-0.5, -0.5), (0.0, 0.0), (0.5, 0.5)]
    );
}

//Q4.2
fn gen_points<R: Rng>(rng: &mut R, n:i64) -> Vec<(f64, f64)> {
    if n <= 0 {return Vec::new()}
    let mut res = Vec::new();
    loop {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        res.push((x,y));
        if res.len() == n as usize {
            break;
        }
    }
    res
}

#[test]
fn test_gen_points() {
    let mut rng = rand::thread_rng();
    assert_eq!(gen_points(&mut rng, 0), vec![]);
    assert_eq!(gen_points(&mut rng, -4), vec![]);
    let a = gen_points(&mut rng, 4);
    assert_eq!(a.len(),4);
    for i in 0..=a.len()-1{
        assert!(a[i].0 >= -1. && a[i].0 <= 1.);
    }
}

//Q4.
