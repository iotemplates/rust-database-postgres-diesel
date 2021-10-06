use postgres::{Client, NoTls, Error};

struct Person {
    _id: i32,
    first_name: String,
    last_name: String
}

fn create_person(first_name: &str, last_name: &str) -> Person {
    Person {
        _id: 0,
        first_name: first_name.to_string(),
        last_name: last_name.to_string()
    }
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/people", NoTls)?;
    
    //create table if not exists
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS person (
            id              SERIAL PRIMARY KEY,
            first_name      VARCHAR NOT NULL,
            last_name      VARCHAR NOT NULL
            )
    ").expect("Failed to create person table!");

    //truncate table
    client.batch_execute("TRUNCATE TABLE person").expect("Failed to truncate person table!");

    let mut people = Vec::new();
    people.push(create_person("John", "Doe"));
    people.push(create_person("Jimmy", "Doe"));
    people.push(create_person("Jane", "Doe"));

    //insert data
    for person in &people {
        client.execute("INSERT INTO person (first_name, last_name) VALUES ($1, $2)", &[&person.first_name, &person.last_name]).expect("Failed to insert into persons!");
    }

    //query
    for row in client.query("SELECT first_name, last_name FROM person", &[]).expect("Failed to query persons!"){
        let person  = create_person(row.get(0), row.get(1));
        println!("First name: {} \t| Last name: {}", person.first_name, person.last_name);
    }

    Ok(())
}
