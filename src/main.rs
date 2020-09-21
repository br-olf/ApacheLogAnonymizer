#[macro_use]
extern crate lazy_static;

use std::fs;
use std::io::{self, Error as ioError, Read};
use std::net::Ipv6Addr;

use clap::{App, Arg};
use regex::Regex;

lazy_static! {
    // This regular expression matches IPv4 addresses.
    static ref RE_IP4: Regex = Regex::new(r"(?x)
        (?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}
        (?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)").unwrap();

    // // This regular expressions match IPv4 addresses with capture groups.
    // // RE_IP4_EXACT considers also line boarders.
    // static ref RE_IP4_EXACT_CG: Regex = Regex::new(r"(?x)
    //     ^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
    //     (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
    //     (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
    //     (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    // static ref RE_IP4_CG: Regex = Regex::new(r"(?x)
    //     (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
    //     (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
    //     (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(?:\.)
    //     (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)").unwrap();
    //
    // // This regular expression matches IPv6 addresses and considers also line boarders.
    // static ref RE_IP6_EXACT: Regex = Regex::new(r"(?x)
    //     ^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$|
    //     ^::(?:[0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}$|
    //     ^[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}$|
    //     ^[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,4}[0-9a-fA-F]{1,4}$|
    //     ^(?:[0-9a-fA-F]{1,4}:){0,2}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,3}[0-9a-fA-F]{1,4}$|
    //     ^(?:[0-9a-fA-F]{1,4}:){0,3}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,2}[0-9a-fA-F]{1,4}$|
    //     ^(?:[0-9a-fA-F]{1,4}:){0,4}[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:)?[0-9a-fA-F]{1,4}$|
    //     ^(?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}::[0-9a-fA-F]{1,4}$|
    //     ^(?:[0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}::$").unwrap();

    // This regular expression matches IPv6 addresses.
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

    // This expression searches for GET parameter in URLs or similar strings
    static ref RE_GETARGS: Regex = Regex::new("(/.+?)(?:\\?[^ '\"]+)+").unwrap();
}


fn main() -> Result<(), ioError> {
    let cli = App::new("ApacheLogAnonymizer")
        .version("0.1")
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

    let mut input = String::new();

    match cli.value_of("in-file") {
        Some(filename) => // option "in-file" is set; try to open file to read
            input = fs::read_to_string(filename)?,
        None =>  // option "in-file" is not set; read from STDIN
            if let Err(e) = io::stdin().read_to_string(&mut input) { return Err(e); },
    }

    let output = anon_get(anon_ipv6(anon_ipv4(input)));

    match cli.value_of("out-file") {
        Some(filename) => // option "out-file" is set; try to open file to write
            return fs::write(filename, output),
        None =>  // option "out-file" is not set; write to STDOUT
            println!("{}", output),
    }

    Ok(())
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

fn anon_get(line: String) -> String {
    let mut anon_str = String::from("");
    let mut last_index: usize = 0;
    for mat in RE_GETARGS.find_iter(&line)
    {
        anon_str.push_str(&line[last_index..mat.start()]);

        anon_str.push_str(&RE_GETARGS.replace(mat.as_str(), "${1}?XXXXX"));

        last_index = mat.end();
    }
    anon_str.push_str(&line[last_index..]);
    anon_str
}