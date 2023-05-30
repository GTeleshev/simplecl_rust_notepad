use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

use super::*;

pub fn insert(note_to: &NOTE) -> Result<()>{
    let conn = Connection::open("notepad.db")?;
    conn.execute(
        "INSERT INTO notepad (subject, contents, date) VALUES (?1, ?2, ?3)",
        (&note_to.subject, &note_to.contents, &note_to.date)
    )?;
    Ok(())
}

pub fn delete(id: i32) -> Result<()>{
    let conn = Connection::open("notepad.db")?;
    println!("Deleting note id = {}", id);
    conn.execute(
        "DELETE FROM notepad WHERE id = ?1",
        [id]
    )?;
    Ok(())
}

pub fn update(note_to: &NOTE) -> Result<()> {
    let conn = Connection::open("notepad.db")?;
    conn.execute(
        "UPDATE notepad SET subject = ?1, contents = ?2 WHERE id = ?3",
        (&note_to.subject, &note_to.contents, note_to.id)
    )?;
    Ok(())
}

pub fn purge() -> Result<()> {
    let conn = Connection::open("notepad.db")?;
    conn.execute(
        "DELETE FROM notepad",
        ()
    )?;
    Ok(())
}

pub fn find_by_date(date: &str) -> Result<()> {
    let conn = Connection::open("notepad.db")?;
    let mut stmt1 = conn.prepare(&*("SELECT id, subject, contents, date FROM notepad WHERE date = '".to_owned() + &*date.to_owned() + &*"'".to_owned()))?;
    let note_iter = stmt1.query_map([], |row| {
        Ok(NOTE {
            id: row.get(0)?,
            subject: row.get(1)?,
            contents: row.get(2)?,
            date: row.get(3)?,
        })
    })?;
    println!("Notes by date: {}", &date);
    for note in note_iter {
            println!("Note found: {:?}", note.unwrap());
    }
    Ok(())
}

pub fn print_all() -> Result<()> {
    let conn = Connection::open("notepad.db")?;
    let mut stmt1 = conn.prepare("SELECT id, subject, contents, date FROM notepad")?;
    let note_iter = stmt1.query_map([], |row| {
        Ok(NOTE {
            id: row.get(0)?,
            subject: row.get(1)?,
            contents: row.get(2)?,
            date: row.get(3)?,
        })
    })?;

    for note in note_iter {
        println!("Note found: {:?}", note.unwrap());
    }
    Ok(())
}

pub fn file_create(file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    Ok(())
}

pub fn file_write(file_name: &str, str_to_write: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;
    write!(file, "{}", str_to_write).expect("oh phoock");
    Ok(())
}

pub fn db_export(file_name: &str) -> Result<()> {
    let conn = Connection::open("notepad.db")?;
    let mut stmt1 = conn.prepare("SELECT id, subject, contents, date FROM notepad")?;
    let note_iter = stmt1.query_map([], |row| {
        Ok(NOTE {
            id: row.get(0)?,
            subject: row.get(1)?,
            contents: row.get(2)?,
            date: row.get(3)?,
        })
    })?;
    file_write(file_name, "[");
    for note in note_iter {
        let mut str1 = note.unwrap().note_to_json();
        file_write(file_name, &str1);
        file_write(file_name, ",\n");
    }
    file_write(file_name, "]");
    Ok(())
}

pub fn create_db() -> Result<()> {
    let conn = Connection::open("notepad.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notepad (
            	ID	INTEGER,
            	subject	TEXT,
            	contents	TEXT,
            	date	TEXT,
            	PRIMARY KEY(ID AUTOINCREMENT)
                )"
        ,
        (), // empty list of parameters.
    )?;
    Ok(())
}