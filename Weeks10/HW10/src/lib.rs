//For question 1
fn vflip(img:&[String]) -> Vec<String> {
    if img.is_empty() {return  Vec::new()}
    let new = img.iter().rev().cloned().collect();
    new
}

fn hflip(img:&[String]) -> Vec<String> { 
    let mut newlst:Vec<String> = Vec::with_capacity(img.len());
    if img.is_empty(){ return Vec::new();}
    let maxvalue = img.iter().map(|s| s.len()).max().unwrap();
    for i in img{
            let mut newi:String = i.chars().rev().collect();
            while newi.len() < maxvalue {
                newi.insert(0, ' ');
            }
            newlst.push(newi)
    }
    newlst
}

#[test] 
fn test_img_flip() { 
    let emp = ["".to_string(); 0]; 
    assert_eq!(vflip(&emp), [""; 0]); 
    assert_eq!(hflip(&emp), [""; 0]); 
 
    let data = [ 
        "<--", 
        "#####", 
        "<==" 
    ].map(|v| v.to_string()); 
 
    assert_eq!( 
        vflip(&data), [ 
            "<==", 
            "#####", 
            "<--" 
        ]); 
    assert_eq!( 
        hflip(&data), [ 
            "  --<", 
            "#####", 
            "  ==<" 
        ]); 
}

//For question 2
fn vcat(img1:&[String], img2:&[String]) -> Vec<String> {
    if img1.is_empty() && img2.is_empty(){return Vec::new();}
    let mut res:Vec<String> = Vec::with_capacity(img1.len()+img2.len());
    for i in img1 {
        res.push(i.to_string());
    }
    for i in img2 {
        res.push(i.to_string());
    }
    res
}

fn hcat(img1:&[String], img2:&[String]) -> Vec<String> {
    if img1.is_empty() && img2.is_empty(){return Vec::new();}
    let maxlen = [img1, img2].iter().map(|x| x.len()).max().unwrap();
    let maxvalue = img1.iter().map(|s| s.len()).max().unwrap();
    let mut newlst:Vec<String> = Vec::with_capacity(maxlen);
    for i in 0..maxlen{
            let mut newi:String = if i < img1.len() {
                img1[i].clone()
            } else{
                String::new()
            };
            let newi2:String = if i < img2.len() {img2[i].to_string()} else {String::new()};
            if newi2 == "".to_string() {}
            else {
                while newi.len() < maxvalue {
                    newi.insert(newi.len(), ' ');
                }
            }
            let ress = newi+&newi2;
            newlst.push(ress)
    }
    newlst
}

#[test] 
fn test_img_cat() { 
    let emp = ["".to_string(); 0]; 
    assert_eq!(vcat(&emp, &emp), [""; 0]); 
    // assert_eq!(hcat(&emp, &emp), [""; 0]); 
 
    let data = [ 
        "<--", 
        "#####", 
        "<==" 
    ].map(|v| v.to_string()); 
    assert_eq!(vcat(&emp, &data), data); 
    assert_eq!(vcat(&data, &emp), data); 
 
    assert_eq!( 
        vcat(&data, &data), [ 
            "<--", 
            "#####", 
            "<==", 
            "<--", 
            "#####", 
            "<==" 
        ]); 
 
    assert_eq!( 
        hcat(&data, &data[..2]), [ 
            "<--  <--", 
            "##########", 
            "<==" 
        ]); 
    assert_eq!( 
        hcat(&data[..2], &data), [ 
            "<--  <--", 
            "##########", 
            "     <==" 
        ]); 
}
