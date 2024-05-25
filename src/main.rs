use db::{handle_command, Table};
use std::collections::BTreeMap;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut tables: BTreeMap<String, Table> = BTreeMap::new();
    loop {
        print!(">>> ");
        io::stdout().flush().expect("Could not write all bytes");

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        let output = handle_command(trimmed, &mut tables);
        println!("{}", output)
    }
}
