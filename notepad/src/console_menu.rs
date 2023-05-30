use std::collections::{BTreeMap};
use std::io::stdin;
use crate::commands;

pub fn console_menu(){
    println!("Записная книжка");
    let mut console_menu: BTreeMap<u32, String> = BTreeMap::new();
    console_menu.insert(1, String::from("Добавление записей"));
    console_menu.insert(2, String::from("Изменение записи"));
    console_menu.insert(3, String::from("Вывод всех записей"));
    console_menu.insert(4, String::from("Импорт"));
    console_menu.insert(5, String::from("Экспорт"));
    console_menu.insert(6, String::from("Удаление записей"));
    console_menu.insert(7, String::from("Поиск по дате"));
    console_menu.insert(8, String::from("Завершить работу"));
    console_menu.insert(9, String::from("Очистка базы"));
    for key in console_menu.keys() {
        println!("{}: {}", key, console_menu[key]);
    }
    loop {
        let mut res = read_console();
        call_function(res);
    }
}

fn call_function(bullet: i32){
    match &bullet {
        1 => commands::create_entry(),
        2 => commands::change_entry(),
        3 => commands::print_entries(),
        4 => commands::import_entries(),
        5 => commands::export_entries(),
        6 => commands::delete_entry(),
        7 => commands::search_by_date(),
        8 => commands::exit_program(),
        9 => commands::purge_entries(),
        _ => console_menu(),
    }
}

pub fn read_console() -> i32 {
    let mut buffer = String::new();
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer);
    let mut res: i32;
    res = match buffer.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => 0,
    };
    return res;
}