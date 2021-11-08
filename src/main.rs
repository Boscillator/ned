mod options;

use std::env;
use std::process;
use options::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = Options::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{}", options.prompt);
}
