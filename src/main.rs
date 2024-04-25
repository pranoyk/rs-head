use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct request {
    filepath: String,
    line_count: u32,
}

fn main() {
    let req = request {
        filepath: "test.txt".to_string(),
        line_count: 10,
    };

    if let Ok(lines) = read_lines(req.filepath) {
        for line in lines.take(req.line_count as usize) {
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