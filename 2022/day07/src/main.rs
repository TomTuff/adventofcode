// this challenge is an obvious candidate for tree data structure
// time to learn about trees!
// i'm going to try to implement my own tree structure with Box<T>
// if I get stuck or solve it, I will then read about trees / their
// implementation in rust, and see if there are any obvious improvements
// I can implement. Probably Rc<> now that I think about it?

use std::fmt::Display;
//use std::borrow::BorrowMut;  // VS CODE REALLY GOT ME HERE
use std::io::{BufReader, BufRead};
use std::fs::File;
use regex;
use std::cell::RefCell;
use std::rc::Rc;

// Using "Doc" instead of the name "File" since std::fs::File is using that name
#[derive(Debug)]
struct Doc {
    name: String,
    size: usize,
}

impl Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
        write!(f, "({}: {})", self.name, self.size)
    }
}

// let's mimic the structure from this article:
// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
#[derive(Debug)]
struct Tree {
    parent: Option<Rc<RefCell<Tree>>>,
    name: String,
    files: Vec<Doc>,
    dirs: Vec<Rc<RefCell<Tree>>>,
}

impl Tree {
    fn cd(self: &Self, dirname: &str) -> Option<&Rc<RefCell<Tree>>> {
        if dirname == ".." {
            self.parent.as_ref()
        } else if dirname == "/" {
            let this_dir = self;
            while let Some(parent_dir) = this_dir.cd("..") {
                if parent_dir.borrow().name == "/" {
                    return Some(parent_dir)
                }
            }
            None
        }else {
            for dir in self.dirs.iter() {
                if dir.borrow().name == dirname {
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
            result = format!("{}{}", &this_dir.borrow().name, result);
        }
        Some(result)
    }

    fn print(&self) -> String {
        return String::from("[")
        + &self
            .dirs
            .iter()
            .map(|tn| format!("{}", tn.borrow().name))
            .collect::<Vec<String>>()
            .join(",")
        + "]";
    }

    fn from_file(file_path: &str) -> Rc<RefCell<Tree>> {   
        // this the object we are filling out
        let root = Rc::new(RefCell::new(Tree {
            parent: None,
            name: "/".to_string(),
            files: vec![],
            dirs: vec![],
        }));

        let mut here = Rc::clone(&root);

        println!("{:?}", root);

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
                //println!("Adding dir here; here's dirs: {:?}", here.borrow().dirs);
                // in case we do ls on the same directory twice, we don't want to double up on the work
                if !here.borrow().dirs.iter().any(|dir| {
                    dir.borrow().name == dir_name
                }) {
                    println!("add {dir_name}");
                    let dir = Rc::new(RefCell::new(Tree {
                        //parent: Some(Rc::clone(&here)),
                        parent: None,
                        name: dir_name.to_owned(),
                        files: vec![],
                        dirs: vec![],
                    }));
                    here.borrow_mut().dirs.push(Rc::clone(&dir));  // VS CODE REALLY GOT ME HERE, LOOK OUT FOR RANDOM IMPORTS
                    {
                      let mut mut_dir = dir.borrow_mut();
                      mut_dir.parent = Some(Rc::clone(&here));
                    }
                }
                //println!("Now, here's dirs: {:?}", here.borrow().dirs);
            }
        }
        
        root
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("???S???");
        let mut result = String::new();
        println!("len self dirs: {:?}", self.dirs.len());
        self.dirs.iter().for_each(|s| {
            if result.len() > 0 {
                result = format!("{}{}", result, ", ");
            }
            result = format!("{}{}", result, s.borrow().name);      
            println!("result: {result}")      
        });
        write!(f, "{}", result)        
    }
}


fn main() {
    let T = Tree::from_file("test_input.txt");
    println!("---");
    println!("{:?}", T.borrow().print());
}
