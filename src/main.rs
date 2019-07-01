mod api;
mod model;
mod prompt;

use api::Handle;
use prompt::*;
use model::Trello;

fn main() {
    println!("Welcome to Trello CLI!");

    let mut t = Trello::new();

    // Main loop
    loop {
        let command = read_command();

        use Command::*;
        match command {
            ListBoards => {
                display_boards(&t);
            }
            Quit => break,
            Help => {
                println!("Try entering 'quit'.");
            },
        }

    }
}