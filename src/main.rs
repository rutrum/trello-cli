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
            List => {
                display_boards(&t);
            },
            MoveBoard(name) => {
                let result = t.update_board(&name);
                match result {
                    Ok(_) => println!("Now in {}.", name),
                    Err(a) => eprintln!("{}", a),
                }
            },
            Pwd => t.print_location(),
            Quit => break,
            Help => {
                println!("Try entering 'quit'.");
            },
        }

    }
}