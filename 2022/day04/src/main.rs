use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error;
use std::process;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct AssignedPair {
    elf1_lowerbound: usize, 
    elf1_upperbound: usize, 
    elf2_lowerbound: usize, 
    elf2_upperbound: usize, 
}

impl AssignedPair {
    fn from_pair_str(pair_str: &str) -> AssignedPair {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        }

        let caps = RE.captures(pair_str).expect("puzzle input guarantees line format");
        assert_eq!(caps.len(), 5); // capture group 0 is the whoel string, groups 1-4 are our values of interest. 
        AssignedPair {
            elf1_lowerbound: caps[1].parse::<usize>().expect("must be an int based on our expression"), 
            elf1_upperbound: caps[2].parse::<usize>().expect("must be an int based on our expression"), 
            elf2_lowerbound: caps[3].parse::<usize>().expect("must be an int based on our expression"), 
            elf2_upperbound: caps[4].parse::<usize>().expect("must be an int based on our expression"), 
        }

    }

    fn determine_if_contained(pair_str: &str) -> bool {
        let pair = AssignedPair::from_pair_str(pair_str);
        println!("pair_str: {pair_str}; pair: {:?}", pair);
        true
    }
}


fn day4_part1(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut contained_pairs = 0usize;

    for line in reader.lines() {
        if AssignedPair::determine_if_contained(&line?) { contained_pairs += 1; }
    }

    Ok(contained_pairs)
}

fn main() {
    let input_file = "real_input.txt";

    let contained_pairs = day4_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day4_part1: {err}");
        process::exit(1);
    });

    println!("Total number of fully contained assignment pairs: {contained_pairs}");
}