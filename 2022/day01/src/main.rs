use std::error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::process;
use std::{thread, time};

fn print_file_contents(input_path: &str) -> Result<(), Box<dyn error::Error>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line?);
        thread::sleep(time::Duration::from_millis(75));
    }

    Ok(())
}

fn main() {
    let input_path = "input2.txt";
    print_file_contents(input_path).unwrap_or_else(|err| {
        println!("Problem printing file contents: {err}");
        process::exit(1);
    });
}
