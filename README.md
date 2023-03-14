# ApacheLogAnonymizer
A small rust replacement for https://www.zendas.de/technik/sicherheit/apache/ which only works for IPv4 addresses.

## Getting stated

```sh
cargo build --release
```

## Usage
```sh
> ./target/debug/ApacheLogAnonymizer -h
ApacheLogAnonymizer 0.1.1
Olaf Pichler <olaf.pichler@urz.uni-heidelberg.de>
Anonymizes web server logs for long term storage in compliance with the GDPR.
By default reads from STDIN and writes to STDOUT.

USAGE:
    ApacheLogAnonymizer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --in-file <in-file>      file to read from instead of STDIN
    -o, --out-file <out-file>    file to redirect output to
```


## A small Rust programm to anonymize webserver logs.

_ApacheLogAnonymizer_ is intended to remove personal data from webserver log files in order to store them in compliance with the GDPR (german: DSGVO).

* _ApacheLogAnonymizer_ uses regular expressions to search for IPv4 addresses, IPv6 addresses and URLs wit GET parameters.
* IP addresses are anonymized by tuncating the second half of all addresses.
* GET parameters are removed and replaced with `XXXXX` to indicate the removal.

_ApacheLogAnonymizer_ was only tested with apache2 `access.log` and `error.log` samples.

---
**This program was written to the best of knowledge and ability but neither functionality nor correctness can be guaranteed.** 
