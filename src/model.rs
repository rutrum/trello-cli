struct Trello {
    boards: Vec<Board>
}

struct Board {
    name: String,
    id: String,
    lists: Vec<List>,
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