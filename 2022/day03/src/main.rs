use std::process;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Rucksack<'a> {
    compartment1: &'a str,
    compartment2: &'a str,
}

impl Rucksack<'_> {
    fn from_str(rucksack_str: &str) -> Rucksack {
        let len = rucksack_str.len();
        Rucksack {
            compartment1: &rucksack_str[0..len/2],
            compartment2: &rucksack_str[len/2..],
        }
    }

    fn find_shared(self: &Self) -> Option<char> {
        Some('a')
    }

    fn priority_from_char(shared_char: &char) -> usize {
        // instead of a huge match statement let's use the u8 representation
        // A - Z -> 065 - 090
        // a - Z -> 097 - 122


        0
    }

    fn priority(self: &Self) -> usize {
        Rucksack::priority_from_char(&self.find_shared().expect("a duplicate is guaranteed by puzzle statement"))
    }
}

fn day3_part1(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut priority = 0usize;

    for line in reader.lines() {
        let line_str = line?;
        priority += Rucksack::from_str(&line_str).priority();
    }

    Ok(priority)
}

fn main() {
    let input_file = "test_input.txt";

    let priority = day3_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day3_part1: {err}");
        process::exit(1);
    });

    println!("Total priority: {priority}");
}
