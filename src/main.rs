mod executor;
mod graph;
mod parser;
mod vertex;

use std::io::{stdin, stdout, Write};

use crate::{
    executor::Executor,
    graph::{GraphFactory, GraphType},
    parser::Parser,
};

fn main() {
    println!("Rust Graph DB Started");
    let mut executor = Executor::new(GraphFactory::new(), GraphType::InMemory);

    loop {
        print!("\nrustgdb> ");
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .expect("Error whilst attempting to read from stdin");

        let command = match Parser::parse(input.trim().to_string()) {
            Ok(command) => command,
            Err(err) => {
                print!("{}", err);
                continue;
            }
        };
        
        let result = executor.execute(command);

        match result {
            Ok(result) => println!("{:?}", result),
            Err(err) => println!("{}", err)
        }
    }
}
