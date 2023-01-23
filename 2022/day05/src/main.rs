use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error;

#[derive(Default)]
struct CrateStacks {
    // we have 9 crate stacks but we will use an array with length 10
    // so we don't have to do -1 on the indexing or any debugging
    cratestacks: [Vec<String>; 10],  
}

impl CrateStacks {
    fn from_file(file_path: &str) -> Option<CrateStacks> {
        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file);

        let mut stack = CrateStacks::default();

        for line in reader.lines() {
            let line_str = line.expect("this should be a valid line 🤔");
            if line_str.len() == 0 { return Some(stack) }
        }

        None
        //CrateStacks { cratestacks: Default::default() }
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
