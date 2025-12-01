use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: only provide the text file");
        return;
    }

    let input_file = &args[1];

    let file = File::open(input_file).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
