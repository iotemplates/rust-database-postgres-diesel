extern crate diesel;
extern crate rust_database_postgres_diesel;

use self::models::*;
use diesel::prelude::*;
use rust_database_postgres_diesel::*;

fn main() {
    use self::schema::persons::dsl::*;

    let connection = establish_connection();
    let results = persons
        .filter(active.eq(true))
        .limit(5)
        .load::<Person>(&connection)
        .expect("Error loading persons");

    println!("Displaying {} persons", results.len());
    for person in results {
        println!("{}", person.first_name);
        println!("-----------\n");
        println!("{}", person.last_name);
    }
}
