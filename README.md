# ApacheLogAnonymizer
## A small Rust programm to anonymize webserver logs.

_ApacheLogAnonymizer_ is intended to remove personal data from webserver log files in order to store them in compliance with the GDPR (german: DSGVO).

* _ApacheLogAnonymizer_ uses regular expressions to search for IPv4 addresses, IPv6 addresses and URLs wit GET parameters.
* IP addresses are anonymized by tuncating the second half of the address.
* GET parameters are removed and replaced with `XXXXX` to indicate the removal.


**_ApacheLogAnonymizer_ was only tested with apache2 `access.log` and `error.log` samples. This program was written to the best of knowledge and ability but neither functionality nor correctness can be guaranteed.**
