// this challenge is an obvious candidate for tree data structure
// time to learn about trees!
// i'm going to try to implement my own tree structure with Box<T>
// if I get stuck or solve it, I will then read about trees / their
// implementation in rust, and see if there are any obvious improvements
// I can implement. Probably Rc<> now that I think about it?

use std::io::{BufReader, BufRead};
use std::fs::File;
use regex;

// Using "Doc" instead of the name "File" since std::fs::File is using that name
#[derive(Debug)]
struct Doc {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Tree<'a> {
    parent: Option<&'a Tree<'a>>,
    name: String,
    files: Vec<Doc>,
    dirs: Vec<Tree<'a>>,
}

impl<'a> Tree<'a> {
    fn cd(self: &Self, dirname: &str) -> Option<&Tree> {
        if dirname == ".." {
            self.parent
        } else {
            for dir in self.dirs.iter() {
                if dir.name == dirname {
                    return Some(&dir);
                }
            }
            None
        }
    }

    fn from_file(file_path: &str) -> Tree {   
        let mut root = Tree {
            parent: None,
            name: "/".to_string(),
            files: vec![],
            dirs: vec![],
        };

        let re_cd = regex::Regex::new(r"\$ cd (.+)").unwrap(); 
        let re_ls = regex::Regex::new(r"\$ ls").unwrap(); 
        let re_dir = regex::Regex::new(r"dir (.+)").unwrap(); 
        let re_doc = regex::Regex::new(r"\d+ (.+)").unwrap(); 

        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file);
        let mut it = reader.lines();

        while let Some(line) = it.next() {
            let line_str = line.unwrap();
            println!("line: {line_str}");
            if let Some(cap) = re_cd.captures(&line_str) {
                let cd_dir = cap.get(1).expect("this regex has a capture group").as_str();
                println!("found cd instruction to dir {cd_dir}");
            } else if re_ls.is_match(&line_str) {
                println!("found ls instruction")
            }
        }
        // for line in reader.lines() {
        //     let line_str = line.expect("Should be a string");

        // }

        root
    }
}

fn main() {
    // // let's try definining a file system..
    // let t = Tree {
    //     name: String::from("/"),
    //     files: vec![
    //         File { name: "tmp.txt".to_string(), size: 12 }, 
    //         File { name: "notes.txt".to_string(), size: 301 }
    //     ],
    //     dirs: vec![
    //         Tree {
    //             name: String::from("home/"),
    //             files: vec![
    //                 File { name: "hello.txt".to_string(), size: 123 }, 
    //                 File { name: "bye.txt".to_string(), size: 101 }
    //             ],
    //             dirs: vec![
    //                 Tree {
    //                     name: String::from("docs/"),
    //                     files: vec![
    //                         File { name: "essay.txt".to_string(), size: 1024 }, 
    //                         File { name: "contacts.txt".to_string(), size: 355 }
    //                     ],
    //                     dirs: vec![],
    //                 }
    //             ],
    //         }
    //     ]
    // };

    let T = Tree::from_file("test_input.txt");
}
