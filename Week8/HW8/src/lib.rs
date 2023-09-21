use csv::{ReaderBuilder, Writer, Trim};
use std::{io::{Read, Write}, error::Error, fs::File};

#[derive(Debug)]
pub struct  Point {
    x:f64,
    y:f64,
}
#[derive(Debug)]
pub struct  PolarPoint {
    r:f64,
    t:f64,
}

//for q1.1 
pub fn to_polar(pt_list: Vec<Point>) -> Vec<PolarPoint> {
    let mut res = Vec::new();
    for i in pt_list {
        let x = i.x as f64; let y =i.y as f64;
        let nr = (x.powf(2.) + y.powf(2.)).sqrt();
        let nt = y.atan2(x);
        res.push(PolarPoint{r: nr as f64, t:nt});
    }
    res
}

//for q1.2
pub fn to_cartesian(pt_list: Vec<PolarPoint>) -> Vec<Point> {
    let mut res = Vec::new();
    for i in pt_list {
        let r = i.r; let t = i.t;
        let nx = r*t.cos();
        let ny = r*t.sin();
        res.push(Point{x:nx, y:ny});
    }
    res
}

//read data in csv for cartesian to polar2.1
pub fn load<R: Read>(rdr: R) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut reader
        = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut output = Vec::new();
    for record in reader.records(){
        if let Ok(rec) = record {
            let x:f64 = rec[0].parse().unwrap();
            let y:f64 = rec[1].parse().unwrap();
            output.push(Point{x:x, y:y});
        }
    }
    Ok(output)
}

//savedata to csv for catesian to polar 2.1 
pub fn savedt<W: std::io::Write>(writer: W, pt_list:Vec<PolarPoint>) -> Result<(), Box<dyn Error>>{
    let mut wtr = Writer::from_writer(writer);
    for i in pt_list{
        wtr.write_record([i.r.to_string(), i.t.to_string()]).unwrap();
    }
    wtr.flush()?;
    Ok(())
}

//for loading point of q2.2 polar to cartesian
pub fn loadtopo<R: Read>(rdr: R) -> Result<Vec<PolarPoint>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
                        .delimiter(b',')
                        .has_headers(false)
                        .trim(Trim::All)
                        .from_reader(rdr);
    let mut output = Vec::new();
    for i in reader.records() {
        if let Ok(rec) =i {
            let r:f64 = rec[0].parse().unwrap();
            let t:f64 = rec[1].parse().unwrap();
            output.push(PolarPoint{r:r, t:t});
        }
    }
    Ok(output)
}

//save for q2.2 polar to cartesian
pub fn savepoint<W: std::io::Write>(write:W, pt_list: Vec<Point>) -> Result<(), Box<dyn Error>>{
    let mut wtr =Writer::from_writer(write);
    for i in pt_list {
        wtr.write_record([i.x.to_string(), i.y.to_string()]).unwrap();
    }
    wtr.flush()?;
    Ok(())
}

//save to html for q3.1 cartesian to polar
pub fn saveashtml(filename:&str , pt_list: Vec<PolarPoint>) -> Result<(), Box<dyn Error>> {
    let mut outfile = File::create(filename)?;
    outfile.write_all(b"<style>\n")?;
    outfile.write_all(b"table, td {\n")?;
    outfile.write_all(b"    border: 1px solid #000000;\n")?;
    outfile.write_all(b"    border-collapse: collapse;\n}\n")?;
    outfile.write_all(b"</style>\n\n")?;
    outfile.write_all(b"<table>\n")?;
    outfile.write_all(b"    <tr>\n")?;
    outfile.write_all(b"        <th>r</th>\n        <th>t</th>\n")?;
    outfile.write_all(b"    </tr>")?;
    for i in pt_list {
        let r = i.r.to_string();
        let t = i.t.to_string();
        outfile.write_all(b"    <tr>\n")?;
        outfile.write_all(format!("        <td>{}</td>\n",r).as_bytes())?;
        outfile.write_all(b"        <td>")?;
        outfile.write_all(t.as_bytes())?;
        outfile.write_all(b"</td>\n")?;
        outfile.write_all(b"    </tr>\n")?;
    }
    outfile.write_all(b"</table>\n")?;
    Ok(())
}

//save for q3.2 polar to cartesian
pub fn saveashtml2(filename:&str , pt_list: Vec<Point>) -> Result<(), Box<dyn Error>> {
    let mut outfile = File::create(filename)?;
    outfile.write_all(b"<style>\n")?;
    outfile.write_all(b"table, td {\n")?;
    outfile.write_all(b"    border: 1px solid #000000;\n")?;
    outfile.write_all(b"    border-collapse: collapse;\n}\n")?;
    outfile.write_all(b"</style>\n\n")?;
    outfile.write_all(b"<table>\n")?;
    outfile.write_all(b"    <tr>\n")?;
    outfile.write_all(b"        <th>x</th>\n        <th>y</th>\n")?;
    outfile.write_all(b"    </tr>")?;
    for i in pt_list {
        let x = i.x.to_string();
        let y = i.y.to_string();
        outfile.write_all(b"    <tr>\n")?;
        outfile.write_all(format!("        <td>{}</td>\n",x).as_bytes())?;
        outfile.write_all(b"        <td>")?;
        outfile.write_all(y.as_bytes())?;
        outfile.write_all(b"</td>\n")?;
        outfile.write_all(b"    </tr>\n")?;
    }
    outfile.write_all(b"</table>\n")?;
    Ok(())
}