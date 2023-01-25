// this challenge is an obvious candidate for tree data structure
// time to learn about trees!
// i'm going to try to implement my own tree structure with Box<T>
// if I get stuck or solve it, I will then read about trees / their
// implementation in rust, and see if there are any obvious improvements
// I can implement. Probably Rc<> now that I think about it?

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Tree {
    name: String,
    files: Vec<File>,
    dirs: Vec<Tree>,
}

fn main() {
    // let's try definining a file system..
    let t = Tree {
        name: String::from("/"),
        files: vec![
            File { name: "tmp.txt".to_string(), size: 12 }, 
            File { name: "notes.txt".to_string(), size: 301 }
        ],
        dirs: vec![
            Tree {
                name: String::from("home/"),
                files: vec![
                    File { name: "hello.txt".to_string(), size: 123 }, 
                    File { name: "bye.txt".to_string(), size: 101 }
                ],
                dirs: vec![
                    Tree {
                        name: String::from("docs/"),
                        files: vec![
                            File { name: "essay.txt".to_string(), size: 1024 }, 
                            File { name: "contacts.txt".to_string(), size: 355 }
                        ],
                        dirs: vec![],
                    }
                ],
            }
        ]
    };

    println!("{:?}", t);
}
