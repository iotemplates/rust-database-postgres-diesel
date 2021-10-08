extern crate diesel;
extern crate rust_database_postgres_diesel;

use rust_database_postgres_diesel::*;

fn main() {
    let connection = establish_connection();
    create_person(&connection, "John", "Doe");
    create_person(&connection, "Jane", "Doe");
}
