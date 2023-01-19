use std::error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::process;

fn day1(input_path: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut this_cal = 0usize;
    let mut this_elf = 0usize;
    let mut max_cal = 0usize;
    let mut max_elf = 0usize;
    let mut line_str = String::new();

    for line in reader.lines() {
        line_str = line?;
        if line_str.len() == 0 {
            this_elf += 1;
            if this_cal > max_cal {
                max_cal = this_cal;
                max_elf = this_elf;
            }
            this_cal = 0;
        } else {
            this_cal += line_str.parse::<usize>()?;
        }
    }

    Ok(max_elf)
}

fn main() {
    let input_path = "input1.txt";
    let which_elf = day1(input_path).unwrap_or_else(|err| {
        println!("Problem during day1: {err}");
        process::exit(1);
    });
    println!("The {which_elf}th elf has the most calories with them")
}
