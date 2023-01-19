use std::error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::process;

fn day1_part1(input_path: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut this_cal = 0usize;
    let mut this_elf = 0usize;
    let mut max_cal = 0usize;

    for line in reader.lines() {
        let line_str = line?;
        if line_str.len() == 0 {
            this_elf += 1;
            if this_cal > max_cal {
                max_cal = this_cal;
            }
            this_cal = 0;
        } else {
            this_cal += line_str.parse::<usize>()?;
        }
    }

    Ok(max_cal)
}

fn day1_part2(input_path: &str) -> Result<usize, Box<dyn error::Error>> {
    Ok(1)
}

fn main() {
    let input_path = "real_input.txt";
    let max_cal = day1_part1(input_path).unwrap_or_else(|err| {
        println!("Problem during day1 part1: {err}");
        process::exit(1);
    });
    println!("The elf with the most calories has {max_cal} calories on them");

    let max_3_cals = day1_part2(input_path).unwrap_or_else(|err| {
        println!("Problem during day1 part1: {err}");
        process::exit(1);
    });
    println!("The 3 elves with the most calories have {max_3_cals} combined calories on them");
}
