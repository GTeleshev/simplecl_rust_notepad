use std::process::exit;
use super::NOTE;

pub fn create_entry() {NOTE::create_note();}
pub fn change_entry() {NOTE::change_note();}
pub fn print_entries() {NOTE::print_notes();}
pub fn import_entries() {NOTE::import_notes();}
pub fn export_entries() {NOTE::export_notes();}
pub fn delete_entry() {NOTE::delete_note();}
pub fn search_by_date() {NOTE::search_by_date();}
pub fn exit_program() {println!("Exiting program"); exit(0)}
pub fn purge_entries() {NOTE::purge_notes();}