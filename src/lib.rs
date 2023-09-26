use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or("~/.data.db".to_string());
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| {
        panic!("Couldn't access database correctly");
    })
}
