//=======================================================================================================================================
//Q1.1

fn min_max_avg(x: &[i64]) -> (i64, i64, f64) {
    if x.is_empty() {
        return (0, 0, 0.)
    }
    let mut min = x[0];
    let mut max = x[0]; //start at 
    let mut avg = 0.;
    let mut iter = x.iter(); 
    while let Some(a) = iter.next() { //if there is a next number it will loop until has no number
        if min > *a {
            min = *a;
        }
        if max < *a {
            max = *a;
        }
        avg += *a as f64

    }
    let avg = avg / x.len() as f64;
    (min, max, avg)

}

#[test]
fn test_min_max_avg() {
    let a = [19, 23, 55, 2, 4, 11];
    let x = [-1,0,1,400];
    assert_eq!(min_max_avg(&a), (2, 55, 19.));
    assert_eq!(min_max_avg(&x), (-1, 400, 100.));
    assert_eq!(min_max_avg(&[]), (0,0,0.));
}

//=======================================================================================================================================
//Q1.2

fn cal_partial_sums(x: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    let mut s = 0;
    for i in 0..x.len() {
        if i == 0 {
            result.push(x[0]);
            s += x[0] //get sum of it go to aniother loop
        } else {
            result.push(x[i] + s);
            s += x[i];
        }
    }
    result
}

#[test]
fn test_cal_partial(){
    let a = [2, 11, 3, 4, 7];
    assert_eq!(cal_partial_sums(&a), vec![2, 13, 16, 20, 27]);
    assert_eq!(cal_partial_sums(&[1,0,-1,-2]), vec![1,1,0,-2]);
    assert_eq!(cal_partial_sums(&[]), vec![]);
}

//=======================================================================================================================================
//Q2.1

fn pack_number_tuple3(x: &[i64], y: &[i64], z: &[i64]) -> Vec<(i64, i64, i64)> {
    let mut result = Vec::new();
    let a = [x.len(), y.len(), z.len()];
    let mut max = a[0];
    for i in 1..a.len() { //for find max len 
        if max < i {
            max = a[i];
        }
    }
    for i in 0..max  {
        let num1 = x.get(i).cloned().unwrap_or(0); //if it blank it will be assume itself as 0
        let num2 = y.get(i).cloned().unwrap_or(0);
        let num3 = z.get(i).cloned().unwrap_or(0);
        result.push((num1, num2, num3));
    }
    result
}

#[test]
fn test_pack_tup3() {
    let a = &[1,2];
    let b = &[4,3];
    let c = &[5];
    assert_eq!(pack_number_tuple3(a, b, c), vec![(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuple3(&[], &[2,5,6], &[7,2]), vec![(0,2,7), (0,5,2), (0,6,0)]);
    assert_eq!(pack_number_tuple3(&[], &[], &[]), vec![]);
    assert_eq!(pack_number_tuple3(&[-2,2,1], &[-1,-2,-3], &[-5,5,0]), vec![(-2,-1,-5), (2,-2,5), (1,-3,0)]);
}

//=======================================================================================================================================
//Q2.2

fn pack_number_tuples_s3(a: &[i64], b: &[i64], c:&[i64]) -> Vec<(i64, i64, i64)>{
    let mut result = Vec::new();
    let x = [a.len(), b.len(), c.len()];

    let mut min = x[0];
    for i in 1..x.len() { //for find minimum len to show out at the result
        if min > x[i] {
            min = x[i];
        }
    }
    for i in 0..min {
        let min1 = a.get(i).cloned().unwrap_or(0);
        let min2 = b.get(i).cloned().unwrap_or(0);
        let min3 = c.get(i).cloned().unwrap_or(0);

        result.push((min1, min2, min3));
    }
    result
}

#[test]
fn test_pack_number_tuples_s3() {
    assert_eq!(pack_number_tuples_s3(&[1,2], &[4,3], &[5]), vec![(1,4,5)]);
    assert_eq!(pack_number_tuples_s3(&[-1,2,0], &[22,3,-2], &[-10]), vec![(-1,22,-10)]);
    assert_eq!(pack_number_tuples_s3(&[], &[], &[-1,2]), vec![]);
    assert_eq!(pack_number_tuples_s3(&[], &[], &[]), vec![]);
}

//=======================================================================================================================================
//Q3.1

fn unpack_number_tuples(x: &[(i64, i64)]) -> (Vec<i64>, Vec<i64>) {
    let mut re1 = Vec::new();
    let mut re2 = Vec::new();
    for i in 0..x.len() { //loop for array
        re1.push(x[i].0); //index array and index tuple 
        re2.push(x[i].1);
    }
    (re1, re2)
}
#[test]
fn test_unpack_number_tuples(){
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(unpack_number_tuples(&[(1, 2), (0,3)]), (vec![1,0], vec![2,3]));
    assert_eq!(unpack_number_tuples(&[(-1, -4), (2, -1), (-3, 6)]), (vec![-1, 2, -3], vec![-4, -1, 6]));
}

//=======================================================================================================================================
//Q3.2

fn unpack_number_tuples3(x: &[(i64, i64, i64)]) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut r1 = Vec::new();
    let mut r2 = Vec::new();
    let mut r3 = Vec::new();
    for i in 0..x.len() {
        r1.push(x[i].0);
        r2.push(x[i].1);
        r3.push(x[i].2);
    }
    (r1, r2, r3)
}

#[test]
fn test_unpack_number_tuples3() {
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(unpack_number_tuples3(&[(-1,2,3)]), (vec![-1], vec![2], vec![3]));
    assert_eq!(unpack_number_tuples3(&[(-2,10,8), (4,5,-1)]), (vec![-2,4], vec![10,5], vec![8, -1]));
    assert_eq!(unpack_number_tuples3(&[(-10,2,3),(-1,-2,-3), (1,2,3), (0,1,0)]), (vec![-10, -1, 1, 0], vec![2, -2, 2, 1], vec![3, -3, 3, 0]));
}