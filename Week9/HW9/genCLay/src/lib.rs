use std::any::type_name;
use std::error::Error;
use std::fs::File;
use std::io::{Write, Read};
use std::thread::available_parallelism;

use rand::Rng;
use csv::{WriterBuilder, ReaderBuilder, Writer, Reader, Trim};

#[derive(Debug)]
pub struct Circle {
    x:f64,
    y:f64,
    r:f64,
}

#[derive(Debug)]
pub struct Layer {
    name:String,
    color:String,
    circle:Vec<Circle>,
}

pub fn gen_obj_layer_list<R: Rng>(mut rng: R, n:i64) -> Vec<Layer> {
    let mut res:Vec<Layer> = Vec::new();
    if n <= 0 {return Vec::new()}
    for i in 1..=n {
        let mut cir = Vec::new();
        let r = rng.gen_range(0..=255);let g = rng.gen_range(0..=255);let b = rng.gen_range(0..=255);let a = rng.gen_range(0..=255);
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
        let numcir = rng.gen_range(20..=50);
        for j in 0..numcir{
            let x = rng.gen_range(-100.0..=100.0); let y = rng.gen_range(-100.0..=100.0); let rad = rng.gen_range(-10.0..=20.0);
            let newcircle = Circle{x:x, y:y, r:rad};
            cir.push(newcircle)
        }
        res.push(Layer{name:format!("Layer {}",i), color:color, circle:cir})
    }
    res
}

#[test]
fn testt() {
    let mut rng = rand::thread_rng();
    assert_eq!(5, gen_obj_layer_list(&mut rng, 5).len());
    for i in gen_obj_layer_list(&mut rng, 5) {
        assert!(i.circle.len() >= 20 && i.circle.len() <= 50);
        for j in &i.circle {
            let x = j.x;
            let y = j.y;
            let r = j.r;
            assert!(x >= -100. && x <= 100. && y >= -100. && y <= 100.);
            assert!(r >= -10. && r<= 20.);
        }
    }
}

//cal average of circle area in each layer
pub fn cal_average_area(lay: &Vec<Layer>) -> Vec<(String, f64)> {
    let mut res = Vec::new();
    for i in lay{
        let mut av = 0.0;
        for j in &i.circle{
            let sum = (j.r as f64).powf(2.0) * std::f64::consts::PI;
            av += sum
        }
        av /= i.circle.len() as f64;
        res.push((i.name.clone(), av));
    }
    res
}

#[test]
fn tresd() {
    let mut rng = rand::thread_rng();
    let a = gen_obj_layer_list(&mut rng, 3);
    let b = cal_average_area(&a);
    assert_eq!(a.len(), b.len());
}


//for save a point from vector layer into csvfile for q2_1
pub fn savefile<W:Write>(write:W, list: Vec<Layer>) -> Result<(), Box<dyn Error>>{
    let mut wtr = WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(write);
    for i in list {
        let mut res = Vec::new();
        for j in i.circle{
            res.push(j.x.to_string());
            res.push(j.y.to_string());
            res.push(j.r.to_string());
        }
        wtr.write_record([i.name, i.color, res.join(",")]).unwrap();
    }
    wtr.flush()?;
    Ok(())
}

//for read file csv and change it into vector for q2_2
pub fn loadfile<R: Read>(rdr: R) -> Result<Vec<Layer>, Box<dyn Error>> {
    let mut read = ReaderBuilder::new()
                                    .delimiter(b',')
                                    .has_headers(false)
                                    .trim(Trim::All)
                                    .from_reader(rdr);
    let mut readresult = Vec::new();
    for i in read.records() {
        if let Ok(r) = i {
            let name = r[0].to_string();
            let color = r[1].to_string();
            let mut epoint = Vec::new();
            // for point in r.iter().skip(2){
            //     let elayer:Vec<f64> = point.split(',').map(|x| x.parse::<f64>().unwrap_or(0.0)).collect();
            //     let mut iter = elayer.iter();
            //     while let Some(x) = iter.next() {
            //         x
            //     }
            let cir:Vec<f64> = r[2].split(',').map(|x| x.parse::<f64>().unwrap_or(0.0)).collect();
            for point in cir.chunks(3){
                let x = point[0];
                let y = point[1];
                let r = point[2];
                let hi = Circle{x, y, r};
                epoint.push(hi);
            }
            // for epot in r.iter().skip(2).collect::<Vec<_>>().chunks(3) {
            //     if epot.len() == 3{
            //         let x = epot[0].parse::<f64>().unwrap();
            //         let y = epot[1].parse::<f64>().unwrap();
            //         let r = epot[2].parse::<f64>().unwrap();
            //         println!("{}",x);
            //         println!("{}",y);
            //         println!("{}",r);
            //         let cir = Circle{x, y, r};
            //         epoint.push(cir);

            //     }
            // }
                // if elayer.len() == 3 { 
                // }
            readresult.push(Layer { name: name, color: color, circle: epoint});
        }
    }
    Ok(readresult)
}

//for save to csv file in q2_2
pub fn savefile1<W:Write>(write:W, list: Vec<(String, f64)>) -> Result<(), Box<dyn Error>>{
    let mut wtr = WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(write);
    for i in list{
        let names = i.0;
        let av = i.1;
        wtr.write_record([names, av.to_string()]);
    }
    wtr.flush()?;
    Ok(())
}

//save as html file in q3_1
pub fn savetohtml1(filename:String, list: Vec<(String, f64)>) -> Result<(), Box<dyn Error>>{
    let mut wtr = File::create(filename)?;
    wtr.write_all(b"<style>\n")?;
    wtr.write_all(b"table, td {\n")?;
    wtr.write_all(b"    border: 1px solid #000000;\n")?;
    wtr.write_all(b"    border-collapse: collapse;\n}\n")?;
    wtr.write_all(b"</style>\n\n")?;
    wtr.write_all(b"<table>\n")?;
    wtr.write_all(b"    <tr>\n")?;
    wtr.write_all(b"        <th>Name</th>\n        <th>Area</th>\n")?;
    wtr.write_all(b"    </tr>")?;
    for i in list {
        let namee = i.0;
        let av = i.1.to_string();
        wtr.write_all(b"    <tr>\n")?;
        wtr.write_all(format!("        <td>{}</td>\n",namee).as_bytes())?;
        wtr.write_all(b"        <td>")?;
        wtr.write_all(av.as_bytes())?;
        wtr.write_all(b"</td>\n")?;
        wtr.write_all(b"    </tr>\n")?;
    }
    wtr.write_all(b"</table>\n")?;
    Ok(())
}

//for q3_2 add find min and max
pub fn cal_average_area_min_max(lay: &Vec<Layer>) -> Vec<(String, f64, f64, f64)> {
    let mut res = Vec::new();
    for i in lay{
        let mut av = 0.0;
        let mut check = Vec::new();
        for j in &i.circle{
            let sum = (j.r as f64).powf(2.0) * std::f64::consts::PI;
            check.push(sum);
            av += sum
        }
        av /= i.circle.len() as f64;
        let mx = check.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let mn = check.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        res.push((i.name.clone(), av, *mx, *mn));
    }
    res
}

//for save to html on q3_2 
pub fn savetohtml2(filename:String, list: Vec<(String, f64, f64, f64)>) -> Result<(), Box<dyn Error>>{
    let mut wtr = File::create(filename)?;
    wtr.write_all(b"<style>\n")?;
    wtr.write_all(b"table, td {\n")?;
    wtr.write_all(b"    border: 1px solid #000000;\n")?;
    wtr.write_all(b"    border-collapse: collapse;\n}\n")?;
    wtr.write_all(b"</style>\n\n")?;
    wtr.write_all(b"<table>\n")?;
    wtr.write_all(b"    <tr>\n")?;
    wtr.write_all(b"        <th>Name</th>\n        <th>Area</th>\n        <th>Max</th>\n        <th>Min</th>\n")?;
    wtr.write_all(b"    </tr>")?;
    for i in list {
        let namee = i.0;
        let av = i.1.to_string();
        let min = i.2.to_string();
        let max = i.3.to_string();
        wtr.write_all(b"    <tr>\n")?;
        wtr.write_all(format!("        <td>{}</td>\n",namee).as_bytes())?;
        wtr.write_all(b"        <td>")?;
        wtr.write_all(av.as_bytes())?;
        wtr.write_all(b"</td>\n")?;
        wtr.write_all(format!("        <td>{}</td>\n        <td>{}</td>\n", max, min).as_bytes())?;
        wtr.write_all(b"    </tr>\n")?;
    }
    wtr.write_all(b"</table>\n")?;
    Ok(())
}