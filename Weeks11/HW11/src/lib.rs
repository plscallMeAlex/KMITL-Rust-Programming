use std::collections::HashMap;
use std::fs::File;
use std::hash::BuildHasher;
use std::io::{BufRead, Write};
use std::{fs, vec};

pub fn make_document(text:&str) -> Vec<String> {
    let res = text.split("\n\n").map(|x| x.to_string()).collect();
    res
}

pub struct Docwithname{
    doc:Vec<String>,
    name: String,
    word: usize,
}

pub fn rank_documents(docs: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut ranked_docs: Vec<Vec<String>> = docs.clone(); // Create a mutable clone of input
    ranked_docs.sort_by(|a, b| b.len().cmp(&a.len())); // Sort by number of paragraphs in descending order
    ranked_docs
}

fn rank_documentswithname(docs: Vec<Docwithname>) -> Vec<Docwithname> {
    let mut ranked_docs = docs; // Create a mutable clone of input
    ranked_docs.sort_by(|a, b| b.doc.len().cmp(&a.doc.len())); // Sort by number of paragraphs in descending order
    ranked_docs
}


#[test]
fn testing() {
    let fox = "The quick brown fox jumps over the lazy dog."; 
    let para3 = "a\n\nb\n\nc"; 
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,-\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
    ";
    let doc1 = make_document(fox);       // 1 paragraph 
    let doc2 = make_document(bustle);    // 2 paragraphs 
    let doc3 = make_document(para3);     // 3 paragraphs 
    
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(docs);

    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}


pub fn loadingfile(name: Vec<String>, filename:String) {
    let mut newdoc = Vec::new();
    for i in name{
        let reading = fs::read_to_string(&i);
        let reading = match reading {
            Ok(ok) => ok.replace("\r\n", "\n"),
            Err(err) => panic!("Failed to read file {}: {err}", i),
        };
        let doc = make_document(&reading);
        let wordcount = countword(&doc);
        newdoc.push(Docwithname{doc, name:i, word:wordcount});
    }
    let ranking = rank_documentswithname(newdoc);

    let mut newreturn:Vec<(String, usize, usize)> = ranking.into_iter().map(|x| (x.name, x.doc.len(), x.word)).collect();
    let mut wtr = File::create(filename+".html").unwrap();
    writeln!(wtr, "<style>");
    writeln!(wtr, "table, td {{");
    writeln!(wtr, "    border: 1px solid #000000;");
    writeln!(wtr, "    border-collapse: collapse;}}");
    writeln!(wtr, "</style>");
    writeln!(wtr, "<table>");
    writeln!(wtr, "    <tr>");
    writeln!(wtr, "        <th>Filename</th>        <th>Paragraph</th>        <th>Word</th>");
    writeln!(wtr, "    </tr>");
    for i in newreturn{
        let fill = i.0;
        let para = i.1;
        let word = i.2;
        writeln!(wtr, "    <tr>").unwrap();
        writeln!(wtr, "        <td>{}</td>", fill).unwrap();
        writeln!(wtr, "        <td>{}</td>", para).unwrap();
        writeln!(wtr, "        <td>{}</td>", word).unwrap();
        writeln!(wtr, "    </tr>").unwrap();
    }
    writeln!(wtr, "</table>");
}

pub fn countword(inp:&Vec<String>) -> usize {
    inp.iter().map(|x| x.split(" ").count()).sum()
}

#[test]
fn testt(){ 
    let fox = "The quick brown fox jumps over the lazy dog."; 
    let para3 = "a\n\nb\n\nc"; 
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,-\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
    ";
    let a = countword(&make_document(fox));
    println!("{}", a)
}