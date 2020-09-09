#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, BufRead};
use std::net::Ipv6Addr;
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


    // // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./access.log") {
    //     // Consumes the iterator, returns an (Optional) String
    //     for line in lines {
    //         if let Ok(content) = line {
    //             println!("{}", anon_ipv4(content));
    //         }
    //     }
    // }

    println!("{}", anon_ipv4(String::from("1.2.3.4 lalala juhuu[43.22.122.253]")));

    // let mut str_seg = String::from("\n");
    // let mut str_oct = String::from("\n");
    // if let Ok(ip6) = &"acdc:1234::fff:1".parse::<Ipv6Addr>() {
    //     let segs = ip6.segments();
    //     for seg in &segs {
    //         str_seg.push_str(&seg.to_string());
    //         str_seg.push(' ');
    //     }
    //     let octs = ip6.octets();
    //     for oct in &octs {
    //         str_oct.push_str(&oct.to_string());
    //         str_oct.push(' ');
    //     }
    //     let anon_v6 = Ipv6Addr::new(segs[0], segs[1], segs[2], segs[3], 0, 0, 0, 0);
    //     println!("{}", anon_v6.to_string());
    // }
    // // println!("{}", str_seg);

    println!("{}", anon_ipv6(String::from("acdc:1234::fff:1 foobar ::1 lala 1:2:3:4:5:6:7:8")));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn anon_ipv4(line: String) -> String {
    let mut anon_str = String::from("");
    let mut last_index: usize = 0;
    for mat in RE_IP4.find_iter(&line)
    {
        anon_str.push_str(&line[last_index..mat.start()]);
        let octets: Vec<&str> = mat.as_str().split('.').collect();

        anon_str.push_str(&octets[0]);
        anon_str.push('.');
        anon_str.push_str(&octets[1]);
        anon_str.push_str(".0.0");

        last_index = mat.end();
    }
    anon_str.push_str(&line[last_index..]);
    anon_str
}

fn anon_ipv6(line: String) -> String {
    let mut anon_str = String::from("");
    let mut last_index: usize = 0;
    for mat in RE_IP6.find_iter(&line)
    {
        anon_str.push_str(&line[last_index..mat.start()]);

        if let Ok(ipv6) = mat.as_str().parse::<Ipv6Addr>() {
            let mut segs = ipv6.segments();

            // replace last 4 segments of IPv6 with 0
            for i in 4..8 {
                segs[i] = 0;
            }
            let anon_ip6 = Ipv6Addr::from(segs);
            anon_str.push_str(&anon_ip6.to_string());
        }
        last_index = mat.end();
    }
    anon_str.push_str(&line[last_index..]);
    anon_str
}