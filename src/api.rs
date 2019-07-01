use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

extern crate serde_json;
extern crate reqwest;

pub struct Handle {
    key: String,
    token: String,
    boards: HashMap<String, String>
}

use std::fmt;
impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "key:{}\ntoken:{}", self.key, self.token)
    }
}

impl Handle {
    // pub fn new(key: String, token: String) -> Handle {
    //     Handle {key, token}
    // }

    pub fn from_file() -> Handle {
        
        // First extract json from keys.json
        let mut f = File::open("./keys.json").unwrap();
        let mut text = String::new();
        f.read_to_string(&mut text).unwrap();

        // Then deserialize and return new handle
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        let key = format!("{}", json["key"].as_str().unwrap());
        let token = format!("{}", json["token"].as_str().unwrap());
        Handle {key, token, boards: HashMap::new()}
    }

    pub fn get_list_boards(&mut self) -> &HashMap<String, String> {
        if self.boards.is_empty() {
            self.find_boards();
        }

        &self.boards
    }

    // Prints list of boards from user
    fn find_boards(&mut self) {
        let url = format!("https://api.trello.com/1/members/me/boards/?key={key}&token={token}",
                key = self.key,
                token = self.token);

        // Make request
        let text = reqwest::get(&url).unwrap().text().unwrap();

        // Deserialize and find all board names
        use serde_json::Value;
        let mut names_ids: HashMap<String, String> = HashMap::new();
        let json: Value = serde_json::from_str(&text).unwrap();
        if let Value::Array(v) = json {
            for board in v {
                if let Value::Object(o) = board {
                    let name = o["name"].as_str().unwrap().to_string();
                    let id = o["id"].as_str().unwrap().to_string();
                    names_ids.insert(name, id);
                }
            }
        }

        self.boards = names_ids;
    }
}