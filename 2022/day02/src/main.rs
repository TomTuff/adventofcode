use std::error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process;

fn play_rpc(values: Vec<char>) -> usize { 0 }

fn day2(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let valid_col1: [char; 3] = ['A', 'B', 'C'];
    let valid_col2: [char; 3] = ['X', 'Y', 'Z'];

    let mut score = 0usize;

    for line in reader.lines() {
        let line_str = line?;

        // we have to make a few assertions here:
        // 1. each line has len 3
        // 2. each line has format <A, B, or C><SPACE><X, Y, or Z>

        if line_str.len() != 3 { 
            return Err(
                // https://doc.rust-lang.org/std/error/trait.Error.html fn from()
                Box::<dyn error::Error + Send + Sync>
                    ::from("A line in the text file had length not equal to three"
                    .to_string())
            ) 
        }

        let mut values = Vec::with_capacity(2);
        // Not doing a loop here because the logic is different on each iteration
        let val1 = line_str.chars().nth(0).expect("We guaranteed the length of this line");
        let val2 = line_str.chars().nth(0).expect("We guaranteed the length of this line");
        let val3 = line_str.chars().nth(0).expect("We guaranteed the length of this line");

        if !valid_col1.contains(&val1) {
            return Err(
                Box::<dyn error::Error + Send + Sync>
                    ::from("A line had col1 value not of A, B, or C"
                    .to_string())
            ) 
        } else {
            values.push(val1);
        }

        if val2 != ' ' {
            return Err(
                Box::<dyn error::Error + Send + Sync>
                    ::from("A line did not have space (' ') as second char"
                    .to_string())
            ) 
        }

        if !valid_col2.contains(&val3) {
            return Err(
                Box::<dyn error::Error + Send + Sync>
                    ::from("A line had col2 value not of X, Y, or Z"
                    .to_string())
            ) 
        } else {
            values.push(val1);
        }

        // We finally guaranteed the data we want to see, now we can play rock paper scissors safely
        score += play_rpc(values)
    }
    Ok(score)
}

fn main() {
    let input_file = "input1.txt";

    let total_score = day2(input_file).unwrap_or_else(|err| {
        println!("Problem during day2: {err}");
        process::exit(1);
    });

    println!("Total score at RPC: {total_score}");
}
