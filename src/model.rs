use super::api::Handle;

pub struct Trello {
    api: Handle,
    boards: Vec<Board>
}

impl Trello {
    pub fn new() -> Trello {
        let mut t = Trello {
            api: Handle::from_file(),
            boards: Vec::new(),
        };
        t.find_boards();
        t
    }

    pub fn find_boards(&mut self) {
        let map = self.api.find_boards();
        for (key, val) in map {
            self.boards.push(Board {
                name: key,
                id: val,
                lists: None
            })
        }
    }

    pub fn get_boards(&self) -> &Vec<Board> {
        &self.boards
    }
}

pub struct Board {
    pub name: String,
    id: String,
    lists: Option<Vec<List>>,
}

struct List {
    name: String,
    cards: Vec<Card>,
}

struct Card {
    name: String,
}

enum Location {
    Trello,
    Board,
    List,
    Card,
}