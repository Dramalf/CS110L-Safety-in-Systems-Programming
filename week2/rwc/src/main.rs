use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    // unimplemented!();

    // Be sure to delete the #[allow(unused)] line above
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .collect()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let lines: Vec<String> = read_file_lines(&filename).expect("Error reading file");
    let contents=lines.join(" ");
    let words: Vec<&str> = contents.split_whitespace().collect();
    let result=format!("{} words;{} lines;{} characters",words.len(),lines.len(),contents.len());
    println!("{}",result);
}
