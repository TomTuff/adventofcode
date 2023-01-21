use std::process;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day3_part1(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut score = 0usize;

    for line in reader.lines() {}

    Ok(0)
}

fn main() {
    let input_file = "test_input.txt";

    let priority = day3_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day3_part1: {err}");
        process::exit(1);
    });

    println!("Total priority: {priority}");
}
