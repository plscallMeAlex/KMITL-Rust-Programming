use XPMImage::{make_xpm2, savefile};

fn main() {
    let ctable = &[
        ("#".into(), "#000000".into()),
        ("-".into(), "#FFFFFF".into()),
    ];
    let rows = ["##--", "##--", "--##", "--##"];
    let pixels: Vec<_> = rows.iter().map(|r| r.to_string()).collect();
    let img = make_xpm2(ctable, &pixels);
    let saving = savefile("Hi".to_string(), img);
}
