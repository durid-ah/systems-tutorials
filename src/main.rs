use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;
use std::io::{stdin, stdout, Write};
use std::process;
use statement_enums::PrepareResult;
use table::Table;
use cursor::TableRef;
use statement::Statement;
use statement_handler::{
    prepare_statement,
    execute_statement,
};

mod path_parser;
mod file_manager;
mod db_config;
mod size_constants;
mod statement_handler;
mod statement_enums;
mod table;
mod pager;
mod statement;
mod row;
mod cursor;

fn parse_meta_command(user_input: &String, table: TableRef) {
    let input = user_input.trim_end();
    match input {
        ".exit" => {
            table.borrow_mut().close_table();
            process::exit(0)
        },
        _ => println!("Unrecognized Meta Command '{}'", input)
    }
}

fn read_path() -> (PathBuf, PathBuf) {
    let mut input = String::new();

    loop {
        println!("Enter Database Location:");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("An error occured reading user input");

        input = String::from(input.trim());
        if input.len() > 0 { break; }
    }
    
    return path_parser::prepare_path(input.trim());
}

fn main() {
    let (file_path, json_path) = read_path();

    let file_mgr = file_manager::FileManager::new(file_path);
    let config = db_config::DBConfig::load(json_path);
    let pager = pager::Pager::open_pager(file_mgr, config);

    let internal_db = Rc::new(RefCell::new(Table::init_table(pager)));

    loop {
        let mut input = String::new();
        print!("db > ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("An error occured reading user input");
        let first_char = input.chars().next().unwrap();

        if first_char == '.' {
            parse_meta_command(&input, Rc::clone(&internal_db));
            continue;
        }

        let mut stmt =  Statement::new();
        match prepare_statement(&input, &mut stmt) {
            PrepareResult::Success => execute_statement(stmt, Rc::clone(&internal_db)),
            PrepareResult::UnrecognizedStatement => println!("Unrecognized statement: \n\t{}", input.trim()),
            PrepareResult::BadStatement(err) => println!("Error: {}", err)
        }
    }    
}
