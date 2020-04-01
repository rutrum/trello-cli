use std::io::{self, Write};
use super::model::{
    Trello, Board, List, Card
};

pub enum Command {
    Quit,
    Help,
    List,
    Pwd,
    MoveBoard(String),
}

pub fn read_command() -> Command {

    print!(":");
    io::stdout().flush().unwrap();

    let mut raw_input = String::new();
    io::stdin().read_line(&mut raw_input).unwrap();

    raw_input = raw_input.trim().to_lowercase();
    let inputs: Vec<&str> = raw_input.split(" ").collect();

    match inputs[0] {
        "q" | "quit"=> Command::Quit,
        "h" | "help" => Command::Help,
        "b" | "board" => {
            if inputs.len() > 1 {
                Command::MoveBoard(inputs[1].to_string())
            } else {
                Command::List
            }
        }
        "p" | "pwd" => Command::Pwd,
        _ => {
            println!("Not a valid command.");
            read_command()
        }
    }

    // if raw_input == "q" || raw_input == "quit" {
    //     Command::Quit
    // } else if raw_input == "h" || raw_input == "help" {
    //     Command::Help
    // } else if raw_input == "b" || raw_input == "board" {
    //     Command::List
    // } else {
    //     println!("Not a valid command.");
    //     read_command()
    // }
    
}

pub fn display_boards(t: &Trello) {
    for board in t.get_boards() {
        println!("{}", board.name);
    }
}