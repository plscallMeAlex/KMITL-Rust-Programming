use HW11::{make_document, rank_documents, loadingfile};

fn main() {
    let inp:Vec<String> = std::env::args().skip(1).collect();
    if inp.len() > 1 {loadingfile(inp, "Output".to_string());} 
    //output.html is the result of this example 
    //cargo run fox.txt bustle.txt para3.txt
}

