use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error;
use std::process;

struct AssignedPairs {}

impl AssignedPairs {
    fn determine_if_contained(pair_str: &str) -> bool {
        true
    }
}


fn day4_part1(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut contained_pairs = 0usize;

    for line in reader.lines() {
        if AssignedPairs::determine_if_contained(&line?) { contained_pairs += 1; }
    }

    Ok(contained_pairs)
}

fn main() {
    let input_file = "test_input.txt";

    let contained_pairs = day4_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day4_part1: {err}");
        process::exit(1);
    });

    println!("Total number of fully contained assignment pairs: {contained_pairs}");
}