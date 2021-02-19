use std::io::{stdin, stdout, Write};
use std::process;

enum StatementType {
    InsertStatement,
    SelectStatement
}

struct Statement {
    st_type: StatementType 
}

fn prepare_statement(command: &String, statement: &mut Statement) -> Result<(), String> {
    let operation = &command[0..6];
    match operation {
        "select" => {
            statement.st_type = StatementType::SelectStatement;
            return Ok(())
        },
        "insert" => {
            statement.st_type = StatementType::InsertStatement;
            return Ok(())
        },
        _ => return Err(String::from("Unrecognized Statement"))
    }
}

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
    }
    
}
