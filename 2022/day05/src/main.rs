use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error;
use regex;

//(\[[A-Z]\]\s|\s{4})

#[derive(Default)]
struct CrateStacks {
    // we have 9 crate stacks but we will use an array with length 10
    // so we don't have to do -1 on the indexing or any debugging
    cratestacks: Vec<Vec<String>>,
    skip_lines: usize, // use this after we build it from a file to skip right to the proper line to pick up the command sequence
}

impl CrateStacks {
    fn from_file(file_path: &str) -> Option<CrateStacks> {
        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file);

        let mut stack = CrateStacks::default();

        let re = regex::Regex::new(r"(\[[A-Z]\]\s|\s{4})").unwrap();

        for line in reader.lines() {
            let line_str = line.expect("this should be a valid line 🤔");
            println!("line {:?}: {line_str}", stack.skip_lines);
            stack.skip_lines += 1;
            if line_str.len() == 0 { return Some(stack) }
            
            for (i, cap) in re.captures_iter(&line_str).enumerate() {
                println!("capture {i}: {:?}", cap);
            }
            
        }

        None
    }

    fn perform_sequence_from_file(self: &mut Self, file_path: &str) {}

    fn top_sequence(self: &Self) -> &str {
        "hi"
    }
}

fn main() {    
    let input_file = "test_input.txt";
    
    let mut stack = CrateStacks::from_file(input_file)
        .expect("puzzle input should guarantee CrateStacks");
    stack.perform_sequence_from_file(input_file);
    let top_crates = stack.top_sequence();

    println!("The sequence of crates at the top of each stack: {top_crates}");
}
