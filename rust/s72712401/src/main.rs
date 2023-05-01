use std::io::{self, BufRead};
use atty::Stream;

fn main() {
    println!("Hello, world!");
    load_stdin();
}

fn load_stdin() {
    if atty::is(Stream::Stdin) {
        println!("HERE");
        return
    }
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        println!("{}", line);
    }
}
