use std::io::{stdin, stdout, Write};
use std::process;

fn parse_input(user_input: &String) {
    let input = user_input.trim_end();
    match input {
        ".exit" => process::exit(0),
        _ => println!("Unrecognized Command '{}'", input)
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("db >");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("An error occured reading user input");
        let first_char = input.chars().next().unwrap();
        
        if first_char == '.' {
            parse_input(&input);
        }
    }
    
}
