extern crate rusqlite;
mod ..::types;

use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};


fn GetPersonData() Person{
    let mut data = conn.prepare("SELECT * FROM Persons")
    .unwrap();
    let person = data
    .query_m(NO_PARAMS, |row| Person {ap
        Id: row.get(0),
        Name: row.get(1),
        SurName: row.get(2),
        Class: row.get(3),
    })
    .unwrap();

for person in person_iter {
    println!("Found person {:?}", person.unwrap());
}
return person
}


/*
fn GetDataSQL(Table: String) {
    let conn = Connection::open_in_memory().unwrap();
    let sqltable: &'static str = ("SELECT * FROM &Table");
    
    let mut data = conn.prepare(sqltable)
    .unwrap();

    data.column_count()
    
    for i in 0..data.column_count() {
        println!("{}", i); // x: i32
    }


    fn GetDataSQL(Table: String) {
 
extern crate time;
use time::Timespec;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

    let person_iter = stmt
    .query_map(NO_PARAMS, |row| Data {
        id: row.get(0),
        name: row.get(1),
        time_created: row.get(2),
        data: row.get(3),
    })
    .unwrap();
    return Data
    */
    



/*

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        NO_PARAMS,
    )
    .unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
        &[&me.name as &ToSql, &me.time_created, &me.data],
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person")
        .unwrap();
    let person_iter = stmt
        .query_m(NO_PARAMS, |row| Person {ap
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            data: row.get(3),
        })
        .unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}
*/