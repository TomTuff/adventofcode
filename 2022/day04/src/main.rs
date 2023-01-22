use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error;
use std::process;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug)]
struct ElfAssignment {
    lower: usize,
    upper: usize,
}

#[derive(Debug)]
struct AssignedPair {
    elf1: ElfAssignment, 
    elf2: ElfAssignment, 
}

impl AssignedPair {
    fn from_pair_str(pair_str: &str) -> AssignedPair {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        }

        let caps = RE.captures(pair_str).expect("puzzle input guarantees line format");
        assert_eq!(caps.len(), 5); // capture group 0 is the whoel string, groups 1-4 are our values of interest. 
        AssignedPair {
            elf1: {
                ElfAssignment { 
                    lower: caps[1].parse::<usize>().expect("must be an int based on our expression"), 
                    upper: caps[2].parse::<usize>().expect("must be an int based on our expression"),
                }
            },
            elf2: {
                ElfAssignment { 
                    lower: caps[3].parse::<usize>().expect("must be an int based on our expression"),
                    upper: caps[4].parse::<usize>().expect("must be an int based on our expression"),
                } 
            },
        }
    }

    fn does_overlap(self: &Self) -> bool {
        //check lower bound
        let lower_elf: &ElfAssignment;
        let upper_elf: &ElfAssignment;
        if self.elf1.lower <= self.elf2.lower {
            lower_elf = &self.elf1;
            upper_elf = &self.elf2;
        } else {
            lower_elf = &self.elf2;
            upper_elf = &self.elf1;
        }

        //compare edge bounds
        (lower_elf.lower <= upper_elf.lower) & (lower_elf.upper >= upper_elf.upper)
    }

    fn does_overlap_from_pair_str(pair_str: &str) -> bool {
        AssignedPair::from_pair_str(pair_str).does_overlap()
    }
}


fn day4_part1(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut contained_pairs = 0usize;

    for line in reader.lines() {
        if AssignedPair::does_overlap_from_pair_str(&line?) { contained_pairs += 1; }
    }

    Ok(contained_pairs)
}

fn main() {
    let input_file = "test_input.txt";

    let contained_pairs = day4_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day4_part1: {err}");
        process::exit(1);
    });

    println!("Total number of fully contained assignment pairs: {contained_pairs}");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1_no_overlap() {
        let pair = AssignedPair {
            elf1: {
                ElfAssignment { 
                    lower: 1,
                    upper: 3,
                }
            },
            elf2: {
                ElfAssignment { 
                    lower: 2,
                    upper: 4,
                } 
            },
        };
        assert!(!pair.does_overlap())
    }

    #[test]
    fn case2_overlap() {
        let pair = AssignedPair {
            elf1: {
                ElfAssignment { 
                    lower: 1,
                    upper: 5,
                }
            },
            elf2: {
                ElfAssignment { 
                    lower: 2,
                    upper: 4,
                } 
            },
        };
        assert!(pair.does_overlap())
    }

    #[test]
    fn case3_overlap_with_lower_equal() {
        let pair = AssignedPair {
            elf1: {
                ElfAssignment { 
                    lower: 1,
                    upper: 5,
                }
            },
            elf2: {
                ElfAssignment { 
                    lower: 1,
                    upper: 4,
                } 
            },
        };
        assert!(pair.does_overlap())
    }

    #[test]
    fn case4_overlap_with_upper_equal() {
        let pair = AssignedPair {
            elf1: {
                ElfAssignment { 
                    lower: 2,
                    upper: 5,
                }
            },
            elf2: {
                ElfAssignment { 
                    lower: 1,
                    upper: 5,
                } 
            },
        };
        assert!(pair.does_overlap())
    }

    #[test]
    fn case5_overlap_with_both_equal() {
        let pair = AssignedPair {
            elf1: {
                ElfAssignment { 
                    lower: 1,
                    upper: 5,
                }
            },
            elf2: {
                ElfAssignment { 
                    lower: 1,
                    upper: 5,
                } 
            },
        };
        assert!(pair.does_overlap())
    }

    #[test]
    fn case6_complete_no_overlap() {
        let pair = AssignedPair {
            elf1: {
                ElfAssignment { 
                    lower: 1,
                    upper: 5,
                }
            },
            elf2: {
                ElfAssignment { 
                    lower: 6,
                    upper: 11,
                } 
            },
        };
        assert!(pair.does_overlap())
    }
}