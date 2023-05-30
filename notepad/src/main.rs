use notepad::{NOTE};
use chrono::{Datelike, Timelike, Utc};

mod console_menu;
mod commands;
mod db;
mod note;


fn main() {
    db::create_db().expect("oops");
    console_menu::console_menu();
}
