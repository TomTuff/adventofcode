use std::error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process;

enum RPC {
    Rock,
    Paper,
    Scissors,
}

impl RPC {
    // no check for character safety because we already checked it in day2().
    // TODO: move that logic here, return Error.
    fn from_char(character: char) -> Option<Self> {
        let rock: [char; 2] = ['A', 'X'];
        let paper: [char; 2] = ['B', 'Y'];
        let scissors: [char; 2] = ['C', 'Z'];
        if rock.contains(&character) { return Some(Self::Rock); } 
        else if paper.contains(&character) { return Some(Self::Paper); }
        else if scissors.contains(&character) { return Some(Self::Scissors); }
        else { return None }
    }

    fn play_rpc(player1: RPC, player2: RPC) -> usize {
        0
    }

    // no check for vector length because we know day2 passes vec with length 2
    fn play_rpc_from_vec(values: Vec<char>) -> usize { 
        Self::play_rpc(RPC::from_char(values[0]).expect("we know we sent a good input from day2()"), 
                       RPC::from_char(values[1]).expect("we know we sent a good input from day2()"))
    }
}

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
        let mut iter = line_str.chars();
        let val1 = iter.nth(0).expect("We guaranteed the length of this line");
        let val2 = iter.nth(0).expect("We guaranteed the length of this line");
        let val3 = iter.nth(0).expect("We guaranteed the length of this line");

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
        score += RPC::play_rpc_from_vec(values)
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
