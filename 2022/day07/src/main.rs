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
        } else if dirname == "/" {
            let mut dir: Option<&Tree> = Some(self);
            while let Some(this_dir) = dir?.cd("..") {
                dir = Some(this_dir)
            }
            return dir;
        }else {
            for dir in self.dirs.iter() {
                if dir.name == dirname {
                    return Some(&dir);
                }
            }
            None
        }
    }

    fn full_path(self: &Self) -> Option<String> {
        let mut result = self.name.to_owned();
        let mut dir = Some(self);
        while let Some(this_dir) = dir?.cd("..") {
            result = format!("{}{}", &this_dir.name, result);
        }
        Some(result)
    }

    fn from_file(file_path: &str) -> Tree {   
        // this the object we are filling out
        let mut root = Tree {
            parent: None,
            name: "/".to_string(),
            files: vec![],
            dirs: vec![],
        };

        // and here's an object we'll use to navigate it
        let mut here = &mut root;

        let re_cd = regex::Regex::new(r"\$ cd (.+)").unwrap(); 
        let re_ls = regex::Regex::new(r"\$ ls").unwrap(); 
        let re_dir = regex::Regex::new(r"dir (.+)").unwrap(); 
        let re_doc = regex::Regex::new(r"\d+ (.+)").unwrap(); 

        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line_str = line.unwrap();
            println!("line: {line_str}");
            if let Some(cap) = re_cd.captures(&line_str) {
                let cd_dir = cap.get(1).expect("this regex has a capture group").as_str();
                println!("found cd instruction to dir {cd_dir}");
            } else if re_ls.is_match(&line_str) {
                println!("found ls instruction")
                // I was thinking that here we should use .next() to find the dir's contents,
                // but then we will go a step to far and there's no way to rewind the iterator.
                // if we see a statement that matches re_dir or re_doc, we just need to know 
                // which directory was listed last.
            } else if let Some(cap) = re_dir.captures(&line_str) {
                let dir_name = cap.get(1).expect("this regex has a capture group").as_str();
                println!("Adding dir here; here's dirs: {:?}", here.dirs);
                // in case we do ls on the same directory twice, we don't want to double up on the work
                if !here.dirs.iter().any(|dir| {
                    dir.name == dir_name
                }) {
                    println!("add {dir_name}");
                    here.dirs.push(Tree {                        
                        parent: Some(here),
                        name: dir_name.to_owned(),
                        files: vec![],
                        dirs: vec![],
                    });
                }
            }
        }
        
        root
    }
}

fn main() {
    let T = Tree::from_file("test_input.txt");
    println!("---");
    println!("{:?}", T);
}
