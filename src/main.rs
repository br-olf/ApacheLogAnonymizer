use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    let re_iv4 = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
    //assert!(re.is_match("2014-01-01"));
    println!("{}", re_iv4.is_match("127.0.0.1"));

    let mut anon_lines: Vec<String> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./access.log") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(content) = line {
                let words: Vec<&str> = content.split(' ').collect();
                let mut anon_line = String::from("");
                for word in words {
                    // println!("{}", re_iv4.is_match(word));
                    if re_iv4.is_match(word) {
                        let ip_parts: Vec<&str> = word.split('.').collect();
                        anon_line.push_str(ip_parts[0]);
                        anon_line.push('.');
                        anon_line.push_str(ip_parts[1]);
                        anon_line.push_str(".0.0");
                    } else {
                        anon_line.push_str(word);
                    }
                    anon_line.push(' ');
                }
                let _anon_line = anon_line;
                println!("{}", _anon_line);
                anon_lines.push(_anon_line);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}