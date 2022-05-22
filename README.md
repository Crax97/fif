# FiF: Find in Files

### Why? 
I often find myself searching for patterns (e.g words, function names) into my company's code base, i decided to develop a small tool to do this task from the cli.

### Usage
`cargo run --release PATTERN [PATH] [-i] [-r]`
fif scans through the entire `PATH` directory tree (by default `.`), scanning each file for lines that match the given `PATTERN`, line by line.
`-i` does case insensitive pattern matching, while `-r` (available only with the `regex` feature) treats the pattern as a regex.

### Optional features
The `regex` features enables pattern matching through regex expressions (using the [regex](https://docs.rs/regex/latest/regex/) library):
to treat the pattern as a regex, pass the `-r` flag.


### Couldn't you just use something else/bash scripting etc...?
Probably, yeah

### ToDo list
* Probably improve readability
* Add releases