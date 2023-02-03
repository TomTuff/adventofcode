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
struct Tree {
    parent: Option<Rc<RefCell<Tree>>>,
    name: String,
    files: Vec<Doc>,
    dirs: Vec<Rc<RefCell<Tree>>>,
}

impl Default for Tree {
    fn default() -> Self {
        Tree {
            parent: None,
            name: "/".to_owned(),
            files: vec![],
            dirs: vec![],
        }
    }
}

// impl Display for Tree {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         //println!("Attempt to display Tree with name {}", self.name);
//         let mut s = "[".to_string();
//         if self.dirs.len() == 0 {               
//             //println!("self.dirs.len() == 0");
//             let l = self.files.len();
//             let mut sep = ", ";
//             for (i, x) in self.files.iter().enumerate() {         
//                 //println!("for loop 1 on i = {i}");
//                 if i + 1 == l { sep = "" }
//                 s = format!("{}{}{}", s, x, sep)
//             }         
//         } else {       
//             //println!("self.dirs.len() != 0");
//             let l = self.files.len();
//             let mut sep = ", ";
//             let mut i = 0;
//             for x in &self.dirs {    
//                 i += 1;
//                 if i == self.dirs.len() {
//                     sep = "";
//                 }
//                 //println!("for loop 2 on x.name = {}", x.borrow().name);
//                 s = format!("{}{} {}{}", s, x.borrow().name, x.borrow(), sep)
//             }    
//         }
//         write!(f, "{}{}", s, "]")
//     }
// }

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //println!("Attempt to display Tree with name {}", self.name);
        let mut s = "[".to_string();     
        let l = self.files.len();
        let mut sep = ", ";
        let mut i = 0;
        for x in &self.dirs {    
            i += 1;
            if i == self.dirs.len() {
                sep = " ";
            }
            //println!("for loop 2 on x.name = {}", x.borrow().name);
            s = format!("{}{} {}{}", s, x.borrow().name, x.borrow(), sep)
        }
        let l = self.files.len();
        let mut sep = ", ";
        for (i, x) in self.files.iter().enumerate() {         
            //println!("for loop 1 on i = {i}");
            if i + 1 == l { sep = "" }
            s = format!("{}{}{}", s, x, sep)    
        }
        write!(f, "{}{}", s, "]")
    }
}

impl Tree {    
    fn cd(self: &mut Self, dirname: &str) -> Option<&Rc<RefCell<Tree>>> {
        //println!("cd with self.name = {}, dirname = {}", self.name, dirname);
        if dirname == ".." {
            return self.parent.as_ref();
        } if dirname == "/" {
            return None 
        }else {
            for dir in self.dirs.iter() {
                if dir.borrow().name == format!("{}/", dirname) {
                    return Some(dir);
                }
            }
            return None;
        }
    }

    fn full_path(self: &Self) -> String {
        let mut result = self.name.clone();
        let parent = &self.parent;
        //println!("write full name for self.name = {}", self.name);
        match parent {
            None => {
                //println!("match None");
                // case where self is root
                //result = format!("/{}", result)
            } 
            Some(ancestor) => {
                //println!("match Some, ancestor.name = {}", &ancestor.borrow().name);
                result = format!("{}{}", ancestor.borrow().name, result);
                
                while let Some(ancestor) = &ancestor.borrow().parent {
                    //print!("found parent with name {}; current value for result: {}", ancestor.borrow().name, result);
                    result = format!("{}{}", ancestor.borrow().name, result)
                }
            }
        }
        result
    }

    fn from_file(file_path: &str) -> Rc<RefCell<Tree>> {   
        // this the object we are filling out
        let root = Rc::new(RefCell::new(Tree::default()));
        println!("root: {}", root.borrow());

        // this is our "perspective" on the tree as we populate it
        let mut here = Rc::clone(&root);

        let re_cd = regex::Regex::new(r"\$ cd (.+)").unwrap(); 
        let re_ls = regex::Regex::new(r"\$ ls").unwrap(); 
        let re_dir = regex::Regex::new(r"dir (.+)").unwrap(); 
        let re_doc = regex::Regex::new(r"(\d+) (.+)").unwrap(); 

        let file = File::open(file_path).expect("we should have this file 🤔");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line_str = line.unwrap();
            println!("line: {line_str}");
            if let Some(cap) = re_cd.captures(&line_str) {
                let cd_dir = cap
                    .get(1)
                    .expect("this regex has a capture group")
                    .as_str();
                println!("found cd instruction to dir {cd_dir}");
                if cd_dir == "/" {
                    // turns out cd / is only in the puzzle input once so this doesn't really matter. we need no expression here.
                } else {
                    // let interim = here.borrow_mut().cd(cd_dir).unwrap();
                    // here = interim;
                    let interim = Rc::clone(here.borrow_mut().cd(cd_dir).unwrap());
                    here = interim;
                }
            } else if re_ls.is_match(&line_str) {
                println!("found ls instruction")
                // I was thinking that here we should use .next() to find the dir's contents,
                // but then we will go a step to far and there's no way to rewind the iterator.
                // if we see a statement that matches re_dir or re_doc, we just need to know 
                // which directory was listed last.
            } else if let Some(cap) = re_dir.captures(&line_str) {
                let dir_name = cap.get(1).expect("this regex has a capture group").as_str();
                // println!("Adding dir here; here's dirs: {:?}", here.borrow().dirs);
                // in case we do ls on the same directory twice, we don't want to double up on the work
                if !here.borrow().dirs.iter().any(|dir| { dir.borrow().name == dir_name }) {
                    println!("add {dir_name}");
                    let dir = Rc::new(RefCell::new(Tree {
                        parent: Some(Rc::clone(&here)),
                        name: format!("{}{}", dir_name, "/").to_owned(),
                        files: vec![],
                        dirs: vec![],
                    }));
                    here.borrow_mut().dirs.push(Rc::clone(&dir));  // VS CODE REALLY GOT ME HERE, LOOK OUT FOR RANDOM IMPORTS
                    println!("here: {}", here.borrow());
                }
            } else if let Some(cap) = re_doc.captures(&line_str) {
                let size = cap
                    .get(1)
                    .expect("this regex has a capture group")
                    .as_str()
                    .parse::<usize>()
                    .expect("this regex capture group ensures digits");
                let name = cap
                    .get(2)
                    .expect("this regex has a capture group")
                    .as_str()
                    .to_string();
                println!("found doc, name = {name}, size = {size}");
                here.borrow_mut().files.push( Doc {
                    name,
                    size,
                });
                println!("{}", here.borrow());
                println!("files len of here: {}", here.borrow().files.len())
            }
        }
        //println!("full path: {}", root.borrow().dirs[0].borrow().full_path());
        root
    }
}


fn main() {
    let T = Tree::from_file("test_input.txt");
    println!("---");
    println!("{}", T.borrow());
}
