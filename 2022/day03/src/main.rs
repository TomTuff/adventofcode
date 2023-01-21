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
        //I could probably google some clever way to do this but let's start with the naive O(n^2) approach
        for item in self.compartment1.chars() {
            let item_as_str: String = vec![item].iter().collect();
            if self.compartment2.contains(&item_as_str) {
                return Some(item)
            }
        }
        None
    }

    fn find_shared_fast(self: &Self) -> Option<char> {
        //let's fully leverage google now
        //https://stackoverflow.com/questions/52882267/how-to-find-if-two-strings-have-common-characters-in-rust   
        const ALPHABET_LEN: usize = 52;
        let mut char_code = 0usize; 
        let mut alphabet = [0; ALPHABET_LEN]; 
        for c in self.compartment1.chars() {
            char_code = Rucksack::priority_from_char(&c).expect("puzzle guarantees a-z or A-Z") - 1;
            alphabet[char_code] += 1; // store each char from first string
        }
        for c in self.compartment2.chars() {
            char_code = Rucksack::priority_from_char(&c).expect("puzzle guarantees a-z or A-Z") - 1;
            if alphabet[char_code] != 0 { // a stored char is found!
                return Some(c);
            }
        }
        None
    }

    fn priority_from_char(shared_char: &char) -> Option<usize> {
        // instead of a huge match statement let's use the u8 representation
        // A - Z -> 065 - 090
        // a - z -> 097 - 122
        let num = *shared_char as u8;  // seems sketch but the puzzle guarantees our input
        if num >= 65 && num <= 090 { Some((num - 64 + 26) as usize) } 
        else if num >= 97 && num <= 122 { Some((num - 96) as usize) }
        else { None }
    }

    fn priority(self: &Self) -> usize {
        Rucksack::priority_from_char(&self.find_shared().expect("a duplicate is guaranteed by puzzle statement"))
            .expect("puzzle guarantees a-z or A-Z")
    }

    fn priority_fast(self: &Self) -> usize {
        Rucksack::priority_from_char(&self.find_shared_fast().expect("a duplicate is guaranteed by puzzle statement"))
            .expect("puzzle guarantees a-z or A-Z")
    }
}

fn day3_part1(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut priority = 0usize;

    for line in reader.lines() {
        let line_str = line?;
        priority += Rucksack::from_str(&line_str).priority();
        // // let's tease out the bug with prints
        // let ruck = Rucksack::from_str(&line_str);
        // println!("ruck: {:?}", ruck);
        // let shared = ruck.find_shared().expect("a duplicate is guaranteed by puzzle statement");
        // println!("shared char: {shared}");
        // let this_priority = Rucksack::priority_from_char(&shared).expect("puzzle guarantees a-z or A-Z");
        // println!("this priority: {this_priority}");
        // priority += this_priority;
    }

    Ok(priority)
}

fn main() {
    let input_file = "real_input.txt";

    let priority = day3_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day3_part1: {err}");
        process::exit(1);
    });

    println!("Total priority: {priority}");
}
