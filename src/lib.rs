#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use models::NewPerson;
use models::Person;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_person(conn: &PgConnection, first_name: &str, last_name: &str) -> Person {
    use schema::persons;

    let new_person = NewPerson { first_name, last_name };

    diesel::insert_into(persons::table)
        .values(&new_person)
        .get_result(conn)
        .expect("Error saving new person")
}
