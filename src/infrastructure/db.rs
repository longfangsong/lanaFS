use rusqlite::Connection;
use std::sync::Mutex;

lazy_static! {
    pub static ref DB: Mutex<Connection> = Mutex::new(Connection::open("./db.sqlite").unwrap());
}