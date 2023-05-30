use std::io::stdin;
use std::time;
use chrono::{Datelike, Timelike, Utc};
pub mod db;
pub mod commands;
pub mod note;
pub mod console_menu;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NOTE {
    pub id: i32,
    pub subject: String,
    pub contents: String,
    pub date: String,
}

impl NOTE {
    pub fn print_note(&self){
        println!("id: {}, subject: {}, contents: {}, date: {}", &self.id, &self.subject, &self.contents, &self.date);
    }
    pub fn note_to_json(&self) -> String {
        let mut str1 = String::new();
        str1 = serde_json::to_string(&self).unwrap();
        str1
    }
    pub fn create_note() {
        let mut subj = String::new();
        let mut cont = String::new();
        let mut buffer = String::new();
        let mut buffer1 = String::new();
        let now1 = Utc::now();
        println!("Enter subject: ");
        stdin().read_line(&mut buffer);
        subj = buffer.trim().parse().unwrap();
        println!("Enter contents: ");
        stdin().read_line(&mut buffer1);
        cont = buffer1.trim().parse().unwrap();
        let mut note1 = NOTE {
            id: 0,
            subject: subj,
            contents: cont,
            date: now1.year().to_string() + "-" + &*now1.month().to_string() + "-" + &*now1.day().to_string(),
        };
        db::insert(&note1);

    }
    pub fn change_note() {
        let mut id1 = 0;
        let mut subj = String::new();
        let mut cont = String::new();
        let mut buffer = String::new();
        let mut buffer1 = String::new();
        let mut buffer2 = String::new();
        let now1 = Utc::now();
        println!("Enter id: ");
        stdin().read_line(&mut buffer);
        id1 = buffer.trim().parse().unwrap();
        println!("Enter subject: ");
        stdin().read_line(&mut buffer1);
        subj = buffer1.trim().parse().unwrap();
        println!("Enter contents: ");
        stdin().read_line(&mut buffer2);
        cont = buffer2.trim().parse().unwrap();
        let mut note1 = NOTE {
            id: id1,
            subject: subj,
            contents: cont,
            date: now1.year().to_string() + "-" + &*now1.month().to_string() + "-" + &*now1.day().to_string(),
        };
        db::update(&note1);
    }
    pub fn print_notes() {
        println!("Entries printed");
        db::print_all().expect("Failed to print...");
    }
    pub fn import_notes() {println!("WIP");}
    pub fn export_notes() {
        let mut file_name = String::new();
        let mut buffer = String::new();
        println!("Enter filename: ");
        stdin().read_line(&mut buffer);
        file_name = buffer.trim().parse().unwrap();
        db::db_export(&file_name);
    }
    pub fn delete_note() {
        println!("Enter ID to delete: ");
        let id1 = console_menu::read_console();
        db::delete(id1);
    }
    pub fn search_by_date() {
        let mut date1 = String::new();
        let mut buffer = String::new();
        println!("Enter date: ");
        stdin().read_line(&mut buffer);
        date1 = buffer.trim().parse().unwrap();
        db::find_by_date(&date1);
    }
    pub fn purge_notes() {
        println!("Purging entries");
        db::purge();
    }
}