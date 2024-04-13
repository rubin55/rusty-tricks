use std::fs::File;
use std::io::stdin;
use std::io::BufReader;
use std::io::prelude::BufRead;
use regex::Regex;
use clap::{Command, Arg, crate_name, crate_description, crate_version};

fn process_lines<T: BufRead + Sized>(r: T, x: Regex) {
    for v in r.lines() {
        let l = v.unwrap();
        match x.find(&l) {
            Some(_) => println!("{}", l),
            None => (),
        }
    }
}

fn main() {
    let args = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::new("pattern")
            .help("the pattern to search for")
            .num_args(1)
            .required(true))
        .arg(Arg::new("input")
            .help("file to search")
            .num_args(1)
            .required(false)
            .default_value("-"))
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let regex = Regex::new(pattern).unwrap();

    let input = args.get_one::<String>("input").unwrap();

    if input == "-" {
        let stdin = stdin();
        let reader = stdin.lock();
        process_lines(reader, regex);
    } else {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        process_lines(reader, regex);
    }
}
