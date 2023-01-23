use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error;
use regex;

//(\[[A-Z]\]\s|\s{4})

#[derive(Default)]
struct CrateStacks {
    stacks: Vec<Vec<String>>,
    num_crates: usize,
}

impl CrateStacks {
    fn from_file(file_path: &str) -> Option<CrateStacks> {
        // make empty stack for us to build up
        let mut stack = CrateStacks::default();

        // find the # of crate stacks
        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file); 
        let re_num_crates = regex::Regex::new(r"\s\d\s$").unwrap();

        for line in reader.lines() {
            let line_str = line.expect("this should be a valid line 🤔");
            if let Some(num_crates) = re_num_crates.find(&line_str) {
                stack.set_num_crates(num_crates
                    .as_str()
                    .trim()
                    .parse::<usize>()
                    .expect("regex guarantees this is a digit"));
            }
        }        


        // read the initial state of the stacks
        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file);
        let re_stack = regex::Regex::new(r"(\[[A-Z]\]\s?|\s{4})").unwrap();

        for line in reader.lines() {
            let line_str = line.expect("this should be a valid line 🤔");
            if line_str.len() == 0 { return Some(stack) }
            
            for (i, cap) in re_stack.captures_iter(&line_str).enumerate() {
                println!("capture {i}: {:?}", cap);
            }
            
        }

        None
    }

    fn set_num_crates(self: &mut Self, num_crates: usize) {
        self.num_crates = num_crates;
        self.stacks = Vec::with_capacity(num_crates);
    }

    fn perform_sequence_from_file(self: &mut Self, file_path: &str) {}

    fn top_sequence(self: &Self) -> &str {
        "hi"
    }
}

fn main() {    
    let input_file = "real_input.txt";
    
    let mut stack = CrateStacks::from_file(input_file)
        .expect("puzzle input should guarantee CrateStacks");
    stack.perform_sequence_from_file(input_file);
    let top_crates = stack.top_sequence();

    println!("The sequence of crates at the top of each stack: {top_crates}");
}
