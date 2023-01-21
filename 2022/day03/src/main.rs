use std::process;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

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
            // we can stop at the first shared char because we are guaranteed only one overlapping item by the puzzle
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

struct Trio<'a> {
    //considered making this fields Rucksacks but the only method we need is a class method (priority from char)
    elf1: &'a str,
    elf2: &'a str,
    elf3: &'a str,
}

impl Trio<'_> {
    fn from_array(elves: &[String; 3]) -> Trio {
        Trio {
            elf1: &elves[0],
            elf2: &elves[1],
            elf3: &elves[2],
        }
    }

    fn find_shared(self: &Self) -> Option<char> {
        //lets see if we can use a similar algorithm from Rucksack::find_shared
        None
    }

    fn priority(self: &Self) -> usize {
        Rucksack::priority_from_char(&self.find_shared().expect("a triplicate is guaranteed by puzzle statement"))
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
    }

    Ok(priority)
}

fn day3_part1_fast(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut priority = 0usize;

    for line in reader.lines() {
        let line_str = line?;
        priority += Rucksack::from_str(&line_str).priority_fast();
    }

    Ok(priority)
}

fn day3_part2(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut priority = 0usize;

    const EMPTY_STRING: String = String::new();
    let mut ruck_array = [EMPTY_STRING; 3];
    let mut idx = 0;

    for line in reader.lines() {
        ruck_array[idx] = line?;
        idx += 1;
        if idx == 3 {
            idx = 0;
            priority += Trio::from_array(&ruck_array).priority();
        }
    }

    Ok(priority)
}

fn main() {
    let input_file = "test_input.txt";

    //SLOW
    let start = Instant::now();
    let priority_slow = day3_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day3_part1: {err}");
        process::exit(1);
    });
    let duration_slow = start.elapsed();

    //FAST
    let start = Instant::now();
    let priority_fast = day3_part1_fast(input_file).unwrap_or_else(|err| {
        println!("Problem during day3_part1: {err}");
        process::exit(1);
    });
    let duration_fast = start.elapsed();


    println!("Total priority, slow algo: {priority_slow}");
    println!("Total priority, fast algo: {priority_fast}");
    println!("Time elapsed for slow: {:?}", duration_slow);
    println!("Time elapsed for slow: {:?}", duration_fast);
    println!("Fast is faster by a factor of {:?}", duration_slow.as_micros() as f64 / duration_fast.as_micros() as f64);

    
    let priority_part2 = day3_part2(input_file).unwrap_or_else(|err| {
        println!("Problem during day3_part2: {err}");
        process::exit(1);
    });
}
