use sqlite::*;

pub struct DatabaseManager {
    connection: Connection
}

impl DatabaseManager {
    pub fn new(addr: &str) -> Self {
        Self {
            connection: open(addr).unwrap()
        }
    }

    fn create_table(&self) {
        let query = format!("CREATE TABLE Games (id TEXT UNIQUE, password TEXT);");
        self.connection.execute(query).unwrap();
    }

    pub fn insert_game(&self, id: &str, password: &str) {
        let query = format!("INSERT INTO Games VALUES ({}, {})", id, password);
        self.connection.execute(query).unwrap();
    }

    pub fn get_game(&self, id: &str) -> Option<String> {
        let query = "SELECT * FROM Games where id = ?";
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((1, id)).unwrap();
        while let Ok(State::Row) = statement.next() {
            if id == &statement.read::<String, _>("id").unwrap() {
                return Some(statement.read::<String, _>("password").unwrap());
            }
        }
        None
    }
}
