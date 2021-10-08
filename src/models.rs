use schema::persons;

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name="persons"]
pub struct NewPerson<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}
