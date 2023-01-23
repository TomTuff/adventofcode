use std::fs::File;
use std::io::{BufRead, BufReader};
use regex;

enum Crane {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Default, Debug)]
struct CrateStacks {
    stacks: Vec<Vec<char>>,
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
        let re_char = regex::Regex::new(r"[A-Z]").unwrap();

        for line in reader.lines() {
            let line_str = line.expect("this should be a valid line 🤔");
            if line_str.len() == 0 { return Some(stack) }
            
            for (i, cap) in re_stack.captures_iter(&line_str).enumerate() {
                if let Some(letter) = re_char.find(&cap[0]) {
                    stack.stacks[i].insert(
                        0, 
                        letter
                            .as_str()
                            .to_string()
                            .as_bytes()[0] as char  //len = 1 guaranteed by the regex re_char!
                    )
                }
            }      
        }

        None
    }

    fn set_num_crates(self: &mut Self, num_crates: usize) {
        self.stacks.clear();
        self.stacks.resize(num_crates, Default::default());
    }

    fn perform_sequence_from_file(self: &mut Self, file_path: &str, crane: Crane) {
        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file);

        let re_command = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap(); //move 5 from 8 to 2

        for line in reader.lines() {
            let line_str = &line.expect("this should be a valid line 🤔");
            if let Some(caps) = re_command.captures(&line_str) {

                // //debug
                // println!("stacks before command = {line_str}\n{:?}", self.stacks);

                let len = caps[1].parse::<usize>().expect("regex guarantees int");
                let mut x = Vec::with_capacity(len);
                let idx_from = caps[2].parse::<usize>().expect("regex guarantees int") - 1;
                let idx_to = caps[3].parse::<usize>().expect("regex guarantees int") - 1;
                let len_to = self.stacks[idx_to].len();
                
                // //debug
                // println!("len: {len},     idx_from: {idx_from},     idx_to: {idx_to},     len_from: {len_from}");

                match crane {
                    Crane::CrateMover9000 => {
                        for _ in 0..len {
                            x.push(self.stacks[idx_from].pop().unwrap());
                        }
                    }
                    Crane::CrateMover9001 => {
                        for _ in 0..len {
                            x.insert(0, self.stacks[idx_from].pop().unwrap());
                        }
                    }
                }

                for _ in  0..len {
                    self.stacks[idx_to].insert(len_to, x.pop().unwrap());
                }

                // //debug
                // println!("x: {:?}", x);
                // println!("stacks after:\n{:?}\n\n\n", self.stacks);
            }
        }

    }

    fn top_sequence(self: &Self) -> String {
        let mut v = Vec::new();
        for idx in 0..self.stacks.len() {
            v.push(*self.stacks[idx].last().unwrap_or_else(|| &' '))
        }
        v.into_iter().collect()
    }
}

fn main() {    
    let input_file = "real_input.txt";
    
    let mut stack1 = CrateStacks::from_file(input_file)
        .expect("puzzle input should guarantee CrateStacks");
    stack1.perform_sequence_from_file(input_file, Crane::CrateMover9000);
    let top_crates1 = stack1.top_sequence();

    println!("The sequence of crates at the top of each stack, if using the CrateMover9000, will be: {top_crates1}");
    
    let mut stack2 = CrateStacks::from_file(input_file)
        .expect("puzzle input should guarantee CrateStacks");
    stack2.perform_sequence_from_file(input_file, Crane::CrateMover9001);
    let top_crates2 = stack2.top_sequence();

    println!("The sequence of crates at the top of each stack, if using the CrateMover9001, will be: {top_crates2}");
}
