use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let line_count = 10;

    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    if let Ok(lines) = read_lines(filepath) {
        for line in lines.take(line_count as usize) {
            if let Ok(ip) = line {
                println!("{}", ip);
            } else {
                println!("### Error while reading line from file. ###");
                break;
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}