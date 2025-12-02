use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: expected filename only");
        return ExitCode::FAILURE;
    }

    let input_file = &args[1];

    let file = match File::open(input_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    ExitCode::SUCCESS
}
