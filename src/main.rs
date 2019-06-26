use std::io::{self, Write};

enum Command {
    Quit,
    Help
}

fn main() {
    println!("Welcome to Trello CLI!");

    // Main loop
    loop {
        let command = read_command();

        use Command::*;
        match command {
            Quit => break,
            Help => {
                println!("Try entering 'quit'.");
            },
        }

    }
}

fn read_command() -> Command {

    print!(":");
    io::stdout().flush().unwrap();

    let mut raw_input = String::new();
    io::stdin().read_line(&mut raw_input).unwrap();

    raw_input = raw_input.trim().to_lowercase();

    if raw_input == "q" || raw_input == "quit" {
        Command::Quit
    } else if raw_input == "h" || raw_input == "help" {
        Command::Help
    } else {
        println!("Not a valid command.");
        read_command()
    }
    
}
