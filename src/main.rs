//!
//! # A small Rust programm to anonymize webserver logs.
//!
//! _ApacheLogAnonymizer_ is intended to remove personal data from webserver log files in order to store them in compliance with the GDPR (german: DSGVO).
//!
//! * _ApacheLogAnonymizer_ uses regular expressions to search for IPv4 addresses, IPv6 addresses and URLs wit GET parameters.
//! * IP addresses are anonymized by tuncating the second half of all addresses.
//! * GET parameters are removed and replaced with `XXXXX` to indicate the removal.
//!
//! _ApacheLogAnonymizer_ was only tested with apache2 `access.log` and `error.log` samples.
//!
//! ---
//! **This program was written to the best of knowledge and ability but neither functionality nor correctness can be guaranteed.**
//!

#[macro_use]
extern crate lazy_static;

use std::fs;
use std::io::{self, Error as ioError, BufRead, BufReader};
use std::net::Ipv6Addr;

use clap::{App, Arg};
use regex::Regex;

lazy_static! {
    /// A regular expression that matches IPv4 addresses.
    static ref RE_IP4: Regex = Regex::new(r"(?x)
        (?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}
        (?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)").unwrap();

    /// A regular expression that matches IPv6 addresses.
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

    /// A regular expression that searches for GET parameters in URLs or similar strings.
    static ref RE_GETARGS: Regex = Regex::new("(/.+?)(?:\\?[^ '\"]+)+").unwrap();
}

/// Parses command line arguments and calls the anon_* functions on input
fn main() -> Result<(), ioError> {
    let cli = App::new("ApacheLogAnonymizer")
        .version("0.1.1")
        .author("Olaf Pichler <olaf.pichler@urz.uni-heidelberg.de>")
        .about("Anonymizes web server logs for long term storage in compliance with the GDPR. \nBy default reads from STDIN and writes to STDOUT. ")
        .arg(
            Arg::with_name("in-file")
                .help("file to read from instead of STDIN")
                .takes_value(true)
                .short("i")
                .long("in-file")
        )
        .arg(
            Arg::with_name("out-file")
                .help("file to redirect output to")
                .takes_value(true)
                .short("o")
                .long("out-file")
        )
        .get_matches();


    let reader: Box<dyn BufRead> = match cli.value_of("in-file") {
        Some(filename) => // option "in-file" is set; try to open file to read
            Box::new(BufReader::new(fs::File::open(filename).unwrap())),
        None =>  // option "in-file" is not set; read from STDIN
            Box::new(BufReader::new(io::stdin()))
    };

    // anonymize input
    let mut output = String::from("");
    for line in reader.lines(){
        output.push_str(&anon_get(anon_ipv6(anon_ipv4(line.unwrap()))));
        output.push('\n');
    }


    match cli.value_of("out-file") {
        Some(filename) => // option "out-file" is set; try to open file to write
            return fs::write(filename, output),
        None =>  // option "out-file" is not set; write to STDOUT
            println!("{}", output),
    }

    Ok(())
}

/// Anonymizes IPv4 Addresses in a given sting by replacing the last 2 octets with 0
fn anon_ipv4(text: String) -> String {
    let mut anon_str = String::from("");
    let mut last_index: usize = 0;
    for mat in RE_IP4.find_iter(&text)
    {
        anon_str.push_str(&text[last_index..mat.start()]);
        let octets: Vec<&str> = mat.as_str().split('.').collect();

        anon_str.push_str(&octets[0]);
        anon_str.push('.');
        anon_str.push_str(&octets[1]);
        anon_str.push_str(".0.0");

        last_index = mat.end();
    }
    anon_str.push_str(&text[last_index..]);
    anon_str
}

/// Anonymizes IPv6 Addresses in a given sting by replacing the last 4 hextets with 0
fn anon_ipv6(text: String) -> String {
    let mut anon_str = String::from("");
    let mut last_index: usize = 0;
    for mat in RE_IP6.find_iter(&text)
    {
        anon_str.push_str(&text[last_index..mat.start()]);

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
    anon_str.push_str(&text[last_index..]);
    anon_str
}

/// Searches for URL + GET parameter like patterns in a given string and replaces the GET parameters with XXXXX
fn anon_get(text: String) -> String {
    let mut anon_str = String::from("");
    let mut last_index: usize = 0;
    for mat in RE_GETARGS.find_iter(&text)
    {
        anon_str.push_str(&text[last_index..mat.start()]);

        anon_str.push_str(&RE_GETARGS.replace(mat.as_str(), "${1}?XXXXX"));

        last_index = mat.end();
    }
    anon_str.push_str(&text[last_index..]);
    anon_str
}