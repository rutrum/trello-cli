use super::api::Handle;
use serde_json::{self, Value};

// Struct that contains the API handler,
// the tree that contains options related to the Trello
// architecture, and a path system that handles moving
// across that architecture
pub struct Trello {
    path: TrelloPath,
    api: Handle,
    boards: Vec<Box<Board>>
}

impl Trello {
    pub fn new() -> Trello {
        let mut t = Trello {
            path: TrelloPath::new(),
            api: Handle::from_file(),
            boards: Vec::new(),
        };
        t.find_boards();
        t
    }

    pub fn find_boards(&mut self) {
        let result = self.api.fetch_boards();
        match result {
            Err(_) => panic!("Could not connect to API."),
            Ok(text) => {
                let json: Value = serde_json::from_str(&text).unwrap();

                if let Value::Array(v) = json {
                    for board in v {
                        if let Value::Object(o) = board {
                            let name = o["name"].as_str().unwrap().to_string();
                            let id = o["id"].as_str().unwrap().to_string();
                            self.boards.push(Box::new(Board::new(name, id)));
                        }
                    }
                }
            },
        }
    }

    pub fn get_boards(&self) -> &Vec<Box<Board>> {
        &self.boards
    }

    pub fn update_board(&mut self, name: &String) -> Result<(), String> {
        if filter(|b| b.name == name, self.boards).count() > 0 {

        }
        self.path.set_board(name)
    }

    pub fn print_location(&self) {
        self.path.print();
    }
}

pub struct Board {
    pub name: String,
    id: String,
    lists: Option<Vec<List>>,
}

impl Board {
    pub fn new(name: String, id: String) -> Board {
        Board {
            name,
            id,
            lists: None,
        }
    }
}

pub struct List {
    name: String,
    cards: Vec<Card>,
}

pub struct Card {
    name: String,
}

enum Location {
    Root, Board, List, Card
}

pub struct TrelloPath {
    board: Option<String>,
    list: Option<String>,
    card: Option<String>,
}

impl TrelloPath {

    fn new() -> Self {
        TrelloPath {
            board: None,
            list: None,
            card: None,
        }
    }

    fn move_down(&mut self, name: &String) -> Result<(), String> {
        match self.board {
            None => self.set_board(name),
            Some(_) => match self.list {
                None => self.set_list(name),
                Some(_) => match self.card {
                    None => self.set_card(name),
                    Some(_) => Err("Already in card.".to_string()),
                }
            }
        }
    }

    pub fn set_board(&mut self, name: &String) -> Result<(), String> {
        self.board = Some(name.to_string());
        self.list = None;
        self.card = None;
        Ok(())
    }

    fn set_list(&mut self, name: &String) -> Result<(), String> {
        match self.board {
            Some(_) => {
                self.list = Some(name.to_string());
                self.card = None;
                Ok(())
            },
            None => Err("No board selected.".to_string()),
        }
    }

    fn set_card(&mut self, name: &String) -> Result<(), String> {
        match (&self.board, &self.list) {
            (Some(_), Some(_)) => {
                self.card = Some(name.to_string());
                Ok(())
            },
            (Some(_), None) => Err("No list selected.".to_string()),
            (None, _) => Err("No board selected.".to_string()),
        } 
    }

    fn location(&self) -> Location {
        if self.board.is_none() {
            Location::Root
        } else if self.list.is_none() {
            Location::Board
        } else if self.card.is_none() {
            Location::List
        } else {
            Location::Card
        }
    }

    fn print(&self) {
        print!("/");
        if let Some(b) = &self.board {
            print!("{}", b);
            if let Some(l) = &self.list {
                print!("/{}", l);
                if let Some(c) = &self.card {
                    print!("/{}", c);
                }
            }
        }
        println!();
    }
}