trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}

impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct PlainText {
    chars: String,
}

struct RepeatedText {
    chars: Box<dyn Text>,
    round: usize,
}

struct JoinedText {
    vecs: Vec<Box<dyn Text>>,
    chr: String,
}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText {
            chars: text.to_string(),
        }
    }
}

impl Text for PlainText {
    fn value(&self) -> String {
        self.chars.clone()
    }
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl Text for RepeatedText {
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(RepeatedText {
            chars: self.chars.clone_box(),
            round: self.round,
        })
    }
    fn value(&self) -> String {
        self.chars.value().repeat(self.round)
    }
}

impl Text for JoinedText {
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(JoinedText {
            vecs: self.vecs.iter().map(|x| x.clone_box()).collect(),
            chr: self.chr.clone(),
        })
    }
    fn value(&self) -> String {
        self.vecs
            .iter()
            .map(|x| x.value())
            .collect::<Vec<String>>()
            .join(&self.chr)
    }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

impl RepeatedText {
    fn with_parts(charss: &dyn Text, num: usize) -> RepeatedText {
        RepeatedText {
            chars: charss.clone_box(),
            round: num,
        }
    }
}

impl JoinedText {
    fn with_parts(txt: Vec<Box<dyn Text>>, chr:String) -> JoinedText {
        JoinedText { vecs: txt.clone(), chr: chr.clone() }
    }
}

#[test]
fn test_text_composition() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);

    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(tvec, t5.value());

    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}

fn main() {
    println!();
}
