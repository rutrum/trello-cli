mod api;
mod model;
mod prompt;

use api::Handle;
use prompt::*;

fn main() {
    println!("Welcome to Trello CLI!");
    let mut api = Handle::from_file();

    // Main loop
    loop {
        let command = read_command();

        use Command::*;
        match command {
            ListBoards => {
                let names = api.get_list_boards();
                for name in names {
                    println!("{}", name.0);
                }
            }
            Quit => break,
            Help => {
                println!("Try entering 'quit'.");
            },
        }

    }
}