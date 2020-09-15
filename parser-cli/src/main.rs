use std::env;
use std::fs;
use std::string::String;

use clippings_parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args.get(1).unwrap()).unwrap();

    let parsed = parse(&input);

    match parsed {
        Ok((_, clippings)) => println!("{:?}", clippings),
        Err(e) => println!("Errors: {:}", e),
    }
}
