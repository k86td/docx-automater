use rusqlite::{Connection, Error};
use dotenvy::dotenv;
use std::env;

pub mod app_state;
pub mod components;
pub mod messages;

// pub fn establish_connection() -> SqliteConnection {
//     dotenv().ok();

//     let db_url = env::var("DATABASE_URL").unwrap_or("~/.data.db".to_string());
//     SqliteConnection::establish(&db_url).unwrap_or_else(|_| {
//         panic!("Couldn't access database correctly");
//     })
// }

#[derive(Debug, PartialEq)]
pub enum DatabaseState {
    Unknown,
    NotCreated,
    Created,
    Disconnected,
    Connected,
    Valid,
    Invalid,
}

pub fn establish_connection() -> Result<Connection, Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite://~/.data.db".to_string());
    Connection::open(db_url)
}