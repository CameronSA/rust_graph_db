mod graph;

use std::io::{stdin, stdout, Write};

fn main() {
    println!("Rust Graph DB Started");

    let mut input = String::new();

    loop {
        print!("rustgdb> ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .expect("Error whilst attempting to read from stdin");
    }
}
