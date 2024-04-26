use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let line_count = 10;

    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    read_lines(filepath, line_count);
}

fn read_lines(filename: &str, line_count: u32) {
    let file = File::open(filename).unwrap_or_else(|e| {
        println!("rs-head: cannot open '{}' for reading: {}", filename, e);
        std::process::exit(1);
    });
    io::BufReader::new(file)
        .lines()
        .take(line_count as usize)
        .for_each(|line| match line {
            Ok(l) => println!("{}", l),
            Err(e) => {
                println!("rs-head: error reading line: {}", e);
                std::process::exit(1);
            }
        });
}
