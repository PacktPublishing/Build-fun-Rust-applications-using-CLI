use std::env;
use std::fs;
use std::process;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: grep_clone <pattern> <filename>");
        process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];

    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        process::exit(1);
    });

    let re = Regex::new(pattern).unwrap_or_else(|err| {
        eprintln!("Invalid regex pattern {}: {}", pattern, err);
        process::exit(1);
    });

    for line in content.lines() {
        if re.is_match(line) {
            println!("{}", line);
        }
    }
}
