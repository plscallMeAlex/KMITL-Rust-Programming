fn main() {
    let inp: Vec<i64> = std::env::args()
        .filter_map(|arg| arg.parse::<i64>().ok())
        .collect();
    let mut result: Vec<(i64, i64, i64)> = Vec::new();
    let mut st = inp[0];
    let ed = inp[1];
    let ad = inp[2];
    let mut count = 0;
    while st < ed {
        result.push((st, st*st, st*st*st));
        st += ad;
        count += 1
    }
    println!("<style>");
    println!("table, td {{");
    println!("  border: 1px solid #000000;");
    println!("  border-collapse: collapse;");
    println!("}}");
    println!("</style>");
    println!();
    println!("<table>");
    println!("  <tr>");
    println!("      <td>x</td>");
    println!("      <td>x^2</td>");
    println!("      <td>x^3</td>");
    println!("  </tr>");
    for i in 0..count {
        println!("  <tr>");
        println!("      <td>{:.1}</td>", result[i].0);
        println!("      <td>{:.1}</td>", result[i].1);
        println!("      <td>{:.1}</td>", result[i].2);
        println!("  </tr>");
    }
    println!("</table>");
}