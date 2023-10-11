use std::error::Error;
use std::fs::File;
use std::io::{BufRead, Write};

#[derive(Debug)]
pub struct Xpm {
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

pub fn make_xpm2(ctable: &[(String, String)], pixels: &[String]) -> Xpm {
    let x = Xpm {
        colors: ctable.to_vec(),
        pixels: pixels.to_vec(),
    };
    x
}

#[test]
fn test_make_xpm2() {
    let ctable = &[
        ("#".into(), "#000000".into()),
        ("-".into(), "#FFFFFF".into()),
    ];

    let rows = ["##--", "##--", "--##", "--##"];
    let pixels: Vec<_> = rows.iter().map(|r| r.to_string()).collect();
    let img = make_xpm2(ctable, &pixels);
    assert_eq!(
        img.colors,
        [
            ("#".into(), "#000000".into()),
            ("-".into(), "#FFFFFF".into())
        ]
    );
    assert_eq!(img.pixels.len(), 4);
    assert_eq!(img.pixels.iter().map(|r| r.len()).max(), Some(4));
    assert_eq!(img.colors.len(), 2);
}

pub fn savefile(name: String, inp: Xpm) {
    let mut create = File::create(name + ".svg").unwrap();
    writeln!(
        create,
        r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">"#
    )
    .unwrap();
    writeln!(create, r#"    <style type="text/css">"#).unwrap();
    writeln!(create, r#"        rect {{ stroke: #00FFFF;  }}"#).unwrap();

    let mut i = 1;
    for color in &inp.colors {
        writeln!(create, r#"        rect.c{} {{ fill: {}; }}"#, i, color.1).unwrap();
        i += 1
    }
    writeln!(create, r#"    </style>"#).unwrap();
    // for j in 0..inp.pixels.len() {
    //     let pix = &inp.pixels[j];
    //     let echar:Vec<String> = pix.chars().map(|b| b.to_string()).collect();
    //     // for i in 0..pix.len() {
    //     //     let ch = &echar[i];
    //     //     for k in &inp.colors {
    //     //         if ch == k.0. {

    //     //         }
    //     //     }
    //     // }

    for y in 0..inp.pixels.len() {
        let row = &inp.pixels[y];
        for x in 0..row.len() {
            let row_chars: Vec<char> = row.chars().collect();
            let pixel = String::from(row_chars[x]);
            let mut class = String::new();
            let mut i = 1;
            for j in &inp.colors {
                if j.0 == pixel {
                    class = format!("c{}", i);
                    break;
                }
                i += 1;
            }

            let x_coor = (x * 20) + 10;
            let y_coor = (y * 20) + 10;

            writeln!(
                create,
                r#"    <rect class="{}" x="{}" y="{}" width="20" height="20" />"#,
                class, x_coor, y_coor
            )
            .unwrap();
        }
    }
    writeln!(create, r#"</svg>"#).unwrap();
}

pub fn read_xpm2<R: BufRead>(reader: &mut R) -> Xpm {
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    let mut ctable: Vec<(String, String)> = Vec::new();
    let mut pixels: Vec<String> = Vec::new();
    let mut current_line = 1;
    for i in lines {
        if current_line < 3 {
            current_line += 1;
            continue;
        }
        let cur_string: Vec<char> = i.chars().collect();

        if cur_string[2] == 'c' {
            let mut color_string = String::new();
            for i in 4..=10 {
                color_string.push(cur_string[i]);
            }
            ctable.push((cur_string[0].to_string(), color_string));
        } else {
            pixels.push(i);
        }
    }

    Xpm {
        colors: ctable,
        pixels,
    }
}

#[test]
fn test_read_xpm2() {
    let checker = "\
! XPM2\n\
4 4 2 1\n\
# c #000000\n\
- c #FFFFFF\n\
##--\n\
##--\n\
--##\n\
--##\n\
";
    let data = checker.as_bytes().to_vec();
    let mut reader = data.as_slice();
    let img = read_xpm2(&mut reader);
    assert_eq!(
        img.colors,
        [
            ("#".into(), "#000000".into()),
            ("-".into(), "#FFFFFF".into())
        ]
    );
    assert_eq!(img.pixels[0].len(), 4);
    assert_eq!(img.pixels.len(), 4);
    assert_eq!(img.colors.len(), 2);
    assert_eq!(img.pixels.iter().map(|r| r.len()).max(), Some(4));
}
