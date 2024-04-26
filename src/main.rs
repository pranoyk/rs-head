use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let matches = Command::new("rs-head")
        .version("version 0.1.0")
        .author("Pranoy Kundu")
        .about("Prints the first few lines of a file")
        .arg(
            Arg::new("filepath")
                .required(true)
                .short('f')
                .long("filepath")
                .help("The file to read"),
        )
        .arg(
            Arg::new("line_count")
                .short('n')
                .long("line_count")
                .help("The number of lines to read")
                .default_value("10"),
        )
        .get_matches();

    let filepath = parse_filepath(&matches);
    let line_count = parse_line_count(&matches);

    read_lines(&filepath, line_count);
}

fn parse_filepath(matches: &clap::ArgMatches) -> String {
    let parse: Option<&String> = matches.get_one("filepath");
    match parse {
        Some(filepath) => return filepath.to_string(),
        None => {
            println!("rs-head: no file path provided");
            std::process::exit(1);
        }
    }
}

fn parse_line_count(matches: &clap::ArgMatches) -> u32 {
    let parse: Option<&String> = matches.get_one("line_count");
    match parse {
        Some(line_count) => {
            return line_count.to_string().parse::<u32>().unwrap_or_else(|e| {
                println!("rs-head: invalid line count, must be a number: {}", e);
                std::process::exit(1);
            })
        }
        None => {
            println!("rs-head: no line count provided");
            std::process::exit(1);
        }
    }
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
