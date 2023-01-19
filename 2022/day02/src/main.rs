use std::error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process;

fn day2(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_str = line?;
    }
    Ok(0)
}

fn main() {
    let input_file = "input1.txt";

    let total_score = day2(input_file).unwrap_or_else(|err| {
        println!("Problem during day2: {err}");
        process::exit(1);
    });

    println!("Total score at RPC: {total_score}");
}
