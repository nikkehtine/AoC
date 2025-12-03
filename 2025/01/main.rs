use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::ExitCode;

const DEBUG: bool = false;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    steps: i32,
}

impl Rotation {
    fn new(direction: Direction, steps: i32) -> Self {
        Rotation { direction, steps }
    }
}

#[derive(Debug, Clone, Copy)]
struct Dial {
    position: i32,
    times_at_zero: i32,
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            times_at_zero: 0,
        }
    }

    fn rotate(&mut self, rotation: Rotation) {
        match rotation.direction {
            Direction::Left => self.position -= rotation.steps,
            Direction::Right => self.position += rotation.steps,
        }

        // Since the dial has 100 numbers (0-99), every 100 steps is a full rotation,
        // so we can ignore the hundreds
        self.position %= 100;

        if self.position < 0 {
            self.position += 100;
        } else if self.position > 99 {
            self.position -= 100;
        } else if self.position == 0 {
            self.times_at_zero += 1;
        }
    }
}

fn decode_line(line: &str) -> Rotation {
    let (direction, distance_str) = line.split_at(1);
    let distance = distance_str.parse::<i32>().unwrap();

    let actual_steps = distance % 100;

    Rotation::new(
        match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        },
        actual_steps,
    )
}

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
    let mut dial = Dial::new();
    if DEBUG {
        println!("{:?}", dial);
        println!("");
    }

    for line in reader.lines() {
        let instruction = &line.unwrap();
        let rotation = decode_line(instruction);
        dial.rotate(rotation);

        if DEBUG {
            println!("{}", instruction);
            println!("{:?}", rotation);
            println!("{:?}", dial);
            println!("");
        }
    }

    println!("{}", dial.times_at_zero);

    ExitCode::SUCCESS
}
