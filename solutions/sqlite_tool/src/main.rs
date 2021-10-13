use rusqlite::{params, Connection, Result};

use std::env;       // to use args passed to main function
use std::process;   //  to use exit function
 

#[derive(Debug)] 
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

const IS_CREATE: i32  = 1>>0; // 0b00000001 = 1 
const IS_READ:   i32  = 2>>0; // 0b00000010 = 2
const IS_UPDATE: i32  = 4>>0; // 0b00000100 = 4
const IS_DELETE: i32  = 8>>0; // 0b00001000 = 8

fn read(){
    println!("read");
}
fn delete(){
    println!("delete");
}
fn update(){
    println!("update");
}
fn create(){
    println!("new");
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    
    debug_print!("{:?}", args);
    let mut selector = 0;
    for input in args {
        match input {
            // here the function take only the first arg and skip doublons
            x if x == "create"  => { if ((selector & IS_READ   ) | (selector & IS_UPDATE ) | (selector & IS_DELETE)) == 0 { selector += IS_CREATE}} ,// by defaut if another flag is on this function retourn error
         _ => println!("Input [{}] does not equal any valid value", input),
        }
    }
    
    debug_print!("{}", selector);
    debug_print!("{:#010b}", selector);
    // verif operation to do, by defaut return error if you pass more one opetion to process
    if selector > 0 {
        if (selector & IS_CREATE) != 0 && (selector ^ IS_DELETE) != 0{
            debug_print!("flags {} or {}",IS_CREATE, IS_DELETE);
        }
        for _input in 0..8 {
            match selector {
                x if (x & IS_CREATE) != 0  => {debug_print!("create") ; selector -= IS_CREATE  }/*create()*/,
                x if (x & IS_READ) != 0  => {debug_print!("read") ; selector -= IS_READ}/*read()*/,
                x if (x & IS_UPDATE) != 0  => {debug_print!("update") ; selector -= IS_UPDATE}/*update()*/,
                x if (x & IS_DELETE) != 0  => {debug_print!("delete") ; selector -= IS_DELETE}/*delete()*/,
                _ => {break},
            }
        }
    }else{ 
        println!("no valid arg found please select one of obligatory arg [create|read|update|delete] [table={{}}] [op={{}}]");
    }
 

    if 1 == 1 {  
        process::exit(1);
    }
   
    create();
    read();
    update();
    delete();


    let conn = Connection::()?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}