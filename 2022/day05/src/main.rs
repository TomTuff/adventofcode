use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error;

struct CrateStacks {
    // we have 9 crate stacks but we will use an array with length 10
    // so we don't have to do -1 on the indexing or any debugging
    cratestacks: [Vec<String>; 10],  
}

impl CrateStacks {
    fn from_file(file_path: &str) -> CrateStacks {
        CrateStacks { cratestacks: Default::default() }
    }

    fn perform_sequence_from_file(self: &mut Self, file_path: &str) {}

    fn top_sequence(self: &Self) -> &str {
        "hi"
    }
}

fn main() {    
    let input_file = "test_input.txt";
    
    let mut stack = CrateStacks::from_file(input_file);
    stack.perform_sequence_from_file(input_file);
    let top_crates = stack.top_sequence()

    println!("The sequence of crates at the top of each stack: {top_crates}");
}
