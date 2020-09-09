#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

lazy_static! {
    // This regular expressions match IPv4 addresses. RE_IP4_EXACT considers also line boarders.
    static ref RE_IP4_EXACT: Regex = Regex::new(r"(?x)
        ^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
        (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
        (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
        (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    static ref RE_IP4: Regex = Regex::new(r"(?x)
        (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
        (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
        (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
        (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)").unwrap();
    // This regular expressions match IPv6 addresses. RE_IP6_EXACT considers also line boarders.
    static ref RE_IP6_EXACT: Regex = Regex::new(r"(?x)
        ^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$|
        ^::(?:[0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}$|
        ^[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}$|
        ^[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,4}[0-9a-fA-F]{1,4}$|
        ^(?:[0-9a-fA-F]{1,4}:){0,2}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,3}[0-9a-fA-F]{1,4}$|
        ^(?:[0-9a-fA-F]{1,4}:){0,3}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,2}[0-9a-fA-F]{1,4}$|
        ^(?:[0-9a-fA-F]{1,4}:){0,4}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:)?[0-9a-fA-F]{1,4}$|
        ^(?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}::[0-9a-fA-F]{1,4}$|
        ^(?:[0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}::$").unwrap();
    static ref RE_IP6: Regex = Regex::new(r"(?x)
        (?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|
        ::(?:[0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}|
        [0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}|
        [0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,4}[0-9a-fA-F]{1,4}|
        (?:[0-9a-fA-F]{1,4}:){0,2}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,3}[0-9a-fA-F]{1,4}|
        (?:[0-9a-fA-F]{1,4}:){0,3}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,2}[0-9a-fA-F]{1,4}|
        (?:[0-9a-fA-F]{1,4}:){0,4}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:)?[0-9a-fA-F]{1,4}|
        (?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}::[0-9a-fA-F]{1,4}|
        (?:[0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}::").unwrap();
}


fn main() {
    println!("Hello, world!");

    //assert!(re.is_match("2014-01-01"));
    //println!("{}", RE_IP4.is_match("127.0.0.1"));

    // let mut anon_lines: Vec<String> = Vec::new();
    // // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./access.log") {
    //     // Consumes the iterator, returns an (Optional) String
    //     for line in lines {
    //         if let Ok(content) = line {
    //             let words: Vec<&str> = content.split(' ').collect();
    //             let mut anon_line = String::from("");
    //             for word in words {
    //                 // println!("{}", re_iv4.is_match(word));
    //                 if RE_IP4.is_match(word) {
    //                     let ip_parts: Vec<&str> = word.split('.').collect();
    //                     anon_line.push_str(ip_parts[0]);
    //                     anon_line.push('.');
    //                     anon_line.push_str(ip_parts[1]);
    //                     anon_line.push_str(".0.0");
    //                 } else {
    //                     anon_line.push_str(word);
    //                 }
    //                 anon_line.push(' ');
    //             }
    //             let anon_line_const = anon_line;
    //             println!("{}", anon_line_const);
    //             anon_lines.push(anon_line_const);
    //         }
    //     }
    // }


    // let anon_access = anon_audit_log_2("./access.log");
    // for line in anon_access {
    //     println!("{}", line)
    // }

    if let Some(cap) = RE_IP4.find(&"lala127.0.0.2") {
        println!("{}", cap.as_str());
    }

    for mat in RE_IP4.find_iter("lala1.1.1.1»foo12.3.2.1gg") {
        println!("{}", mat.as_str());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn anon_audit_log_1<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>, {
    let mut anon_lines: Vec<String> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(content) = line {
                let words: Vec<&str> = content.split(' ').collect();
                let mut anon_line = String::from("");
                for word in words {
                    if RE_IP4.is_match(word) {
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
                let anon_line_const = anon_line;
                anon_lines.push(anon_line_const);
            }
        }
    }
    anon_lines
}

fn anon_audit_log_2<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>, {
    let mut anon_lines: Vec<String> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for r_line in lines {
            if let Ok(line) = r_line {
                let o_mat = RE_IP4.find(&line);
                match o_mat {
                    Some(mat) => {
                        let mut anon_str = String::from("");
                        anon_str.push_str(&line[..mat.start()]);

                        for cap in RE_IP4.captures_iter(&line) {
                            // println!("{}", &cap.len());
                            anon_str.push_str(&cap[1]);
                            anon_str.push('.');
                            anon_str.push_str(&cap[2]);
                            anon_str.push_str(".0.0");
                            anon_str.push_str(&line[mat.end()..]);
                        }
                        anon_str.push_str(&line[..mat.start()]);
                        anon_lines.push(anon_str);
                    }
                    None => {
                        anon_lines.push(line);
                    }
                }
            }
        }
    }
    anon_lines
}

fn anon_access_line(line: String) -> String {
    let o_mat = RE_IP4.find(&line);
    let mut anon_str = String::from("");
    match o_mat {
        Some(mat) => {
            anon_str.push_str(&line[..mat.start()]);

            for cap in RE_IP4.captures_iter(&line) {
                // println!("{}", &cap.len());
                anon_str.push_str(&cap[1]);
                anon_str.push('.');
                anon_str.push_str(&cap[2]);
                anon_str.push_str(".0.0");
                anon_str.push_str(&line[mat.end()..]);
            }
            anon_str.push_str(&line[..mat.start()]);
        }
        None => {
            anon_str.push_str(line.as_str());
        }
    }
    anon_str
}