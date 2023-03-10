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

enum Overlap {
    Partial,
    Complete,
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

    fn does_overlap_combo(self: &Self, overlap: Overlap) -> bool {
        //check lower bound  lower_elf 1/5,  upper_elf 6/11
        let lower_elf: &ElfAssignment;
        let upper_elf: &ElfAssignment;
        if self.elf1.lower < self.elf2.lower {
            lower_elf = &self.elf1;
            upper_elf = &self.elf2;
        } else if self.elf1.lower > self.elf2.lower {
            lower_elf = &self.elf2;
            upper_elf = &self.elf1;
        } else { // this was the bug, lower bounds being equal necessitates checking the upper bound
            if self.elf1.upper >= self.elf2.upper {
                lower_elf = &self.elf1;
                upper_elf = &self.elf2;
            } else {
                lower_elf = &self.elf2;
                upper_elf = &self.elf1;                
            }
        }

        // //debug
        // println!("the pair: {:?}", self);
        // println!("lower elf: {:?}", lower_elf);
        // println!("upper elf: {:?}", upper_elf);
        // println!("expression 1: {:?}", lower_elf.lower <= upper_elf.lower);
        // println!("expression 2: {:?}", lower_elf.upper >= upper_elf.upper);

        //compare edge bounds
        match overlap {
            Overlap::Complete => { lower_elf.upper >= upper_elf.upper },
            Overlap::Partial => { lower_elf.upper >= upper_elf.lower },
        }
        
    }

    fn does_overlap_complete(self: &Self) -> bool {
        self.does_overlap_combo(Overlap::Complete)
    }

    fn does_overlap_partial(self: &Self) -> bool {
        self.does_overlap_combo(Overlap::Partial)
    }

    fn does_overlap_complete_from_pair_str(pair_str: &str) -> bool {
        AssignedPair::from_pair_str(pair_str).does_overlap_complete()
    }

    fn does_overlap_partial_from_pair_str(pair_str: &str) -> bool {
        AssignedPair::from_pair_str(pair_str).does_overlap_partial()
    }
}


fn day4_part1(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut contained_pairs = 0usize;

    for line in reader.lines() {
        if AssignedPair::does_overlap_complete_from_pair_str(&line?) { contained_pairs += 1; }
    }

    Ok(contained_pairs)
}


fn day4_part2(input_file: &str) -> Result<usize, Box<dyn error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut contained_pairs = 0usize;

    for line in reader.lines() {
        if AssignedPair::does_overlap_partial_from_pair_str(&line?) { contained_pairs += 1; }
    }

    Ok(contained_pairs)
}

fn main() {
    let input_file = "real_input.txt";

    let contained_pairs = day4_part1(input_file).unwrap_or_else(|err| {
        println!("Problem during day4_part1: {err}");
        process::exit(1);
    });

    println!("Total number of fully contained assignment pairs: {contained_pairs}");

    let partial_overlap_pairs = day4_part2(input_file).unwrap_or_else(|err| {
        println!("Problem during day4_part2: {err}");
        process::exit(1);
    });

    println!("Total number of partially overlapping assignment pairs: {partial_overlap_pairs}");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_case1_no_overlap() {
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
        assert!(!pair.does_overlap_complete());
        let pair_swap = AssignedPair {
            elf1: pair.elf2,
            elf2: pair.elf1,
        };
        assert!(!pair_swap.does_overlap_complete());
    }

    #[test]
    fn part1_case2_overlap() {
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
        assert!(pair.does_overlap_complete());
        let pair_swap = AssignedPair {
            elf1: pair.elf2,
            elf2: pair.elf1,
        };
        assert!(pair_swap.does_overlap_complete());
    }

    #[test]
    fn part1_case3_overlap_with_lower_equal() {
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
        assert!(pair.does_overlap_complete());
        let pair_swap = AssignedPair {
            elf1: pair.elf2,
            elf2: pair.elf1,
        };
        assert!(pair_swap.does_overlap_complete());
    }

    #[test]
    fn part1_case4_overlap_with_upper_equal() {
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
        assert!(pair.does_overlap_complete());
        let pair_swap = AssignedPair {
            elf1: pair.elf2,
            elf2: pair.elf1,
        };
        assert!(pair_swap.does_overlap_complete());
    }

    #[test]
    fn part1_case5_overlap_with_both_equal() {
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
        assert!(pair.does_overlap_complete());
        let pair_swap = AssignedPair {
            elf1: pair.elf2,
            elf2: pair.elf1,
        };
        assert!(pair_swap.does_overlap_complete());
    }

    #[test]
    fn part1_case6_complete_no_overlap() {
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
        assert!(!pair.does_overlap_complete());
        let pair_swap = AssignedPair {
            elf1: pair.elf2,
            elf2: pair.elf1,
        };
        assert!(!pair_swap.does_overlap_complete());
    }
}