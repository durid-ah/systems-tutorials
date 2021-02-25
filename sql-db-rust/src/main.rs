use std::io::{stdin, stdout, Write};
use std::process;
use statement_enums::StatementType;
use statement_handler::{
    Statement,
    prepare_statement,
};

mod statement_handler;
mod statement_enums;


fn parse_meta_command(user_input: &String) {
    let input = user_input.trim_end();
    match input {
        ".exit" => process::exit(0),
        _ => println!("Unrecognized Command '{}'", input)
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("db > ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("An error occured reading user input");
        let first_char = input.chars().next().unwrap();

        if first_char == '.' {
            parse_meta_command(&input);
            continue;
        }

        let mut stmt =  Statement::new();
        match prepare_statement(&input, &mut stmt) {
            _ => println!("Stuff")
        }
    }    
}
