use rand::Rng;
#[derive(Debug, PartialEq)]
pub struct Point {
    x:i64,
    y:i64,
}

#[derive(Debug)]
pub struct Layer {
    name:String,
    color:String,
    points:Vec<Point>,
}

fn gen_layer<R: Rng>(rng: &mut R, name:String, color:String) -> Layer { 
    let round = rng.gen_range(20..=50);
    let mut res = Vec::new();
    for i in 0..round {
        let x = rng.gen_range(-100..=100);
        let y = rng.gen_range(-100..=100);
        let point = Point{x, y};
        res.push(point);        
    }
    Layer{
        name:name,
        color:color,
        points:res,
    }

}

#[test]
fn test3_1() {
    let mut rng = rand::thread_rng();
    let layer = gen_layer(&mut rng, "dssds".to_string(), "#223e3".to_string());
    let inp = &layer.points;    
    let mut count = 0;
    for i in inp {
        assert!(i.x >= -100 && i.x <= 100);
        assert!(i.y >= -100 && i.y <= 100);
        count += 1
    }
    assert_eq!(count,inp.len());
}

pub fn gen_layer_list<R: Rng>(rng: &mut R, n:i64) -> Vec<Layer> {
    let mut res2 = Vec::new();
    for i in 1..=n{
        let mut res =Vec::new();
        let r = rng.gen_range(0..=255);let g = rng.gen_range(0..=255);let b = rng.gen_range(0..=255);let a = rng.gen_range(0..=255);
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
        let x = rng.gen_range(-100..=100);let y = rng.gen_range(-100..=100);
        let point = Point{x, y};
        res.push(point);
        res2.push(Layer{name:format!("Layer  {i}"), color:color.to_string(), points:res});
    }
    res2
}

#[test]
fn test3_2() {
    let mut rng = rand::thread_rng();
    let llayer = gen_layer_list(&mut rng, 3);
    let mut count = 0;
    for i in llayer {
        count += 1;
        let coor = i.points;
        for j in coor{
            assert!(j.x <= 100 && j.x >= -100);
            assert!(j.y <= 100 && j.y >= -100);
        }
        assert_eq!(i.color.len(), 9);
        assert_eq!(i.name, format!("Layer  {count}"));
    }
    println!("{:?}", gen_layer_list(&mut rng, 3));
}

