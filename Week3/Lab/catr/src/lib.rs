use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,

}

pub fn get_args() -> MyResult<Config> {
    let matches
        = App::new("cats")
                .version("0.1.0")
                .author("SE15 <se15@kmitl.ac.th>")
                .about("Rust cat")
                .arg(
                    Arg::with_name("number_nonblank")
                        .short("b")
                        .long("number-nonblank")
                        .help("Number non-blank lines")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .help("Number lines")
                        .takes_value(false)
                        .conflicts_with("number_nonblank"),
                )
                .arg(
                     Arg::with_name("files")
                        .value_name("FILE")
                        .help("Input a file")
                        .multiple(true)
                        .default_value("-"),
                ).get_matches();

    let files: Vec<String> = matches
        .values_of_lossy("files")
        .unwrap() 
        .into_iter()
        .map(|f| f.to_string())
        .collect();

    Ok(Config {
        files,//: matches.value_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
                
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines{
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines{
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    }else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
    

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
    "-" => Ok(Box::new( BufReader::new(io::stdin()) )),
    _ => Ok(Box::new( BufReader::new(File::open(filename)?) )),
    }
}
