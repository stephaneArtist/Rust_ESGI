use std::io::{self, Write};

pub fn read_line(print_message: &str, s: &mut String) {
    print!("{}", print_message);
    io::stdout().flush().expect("Can't flush");

    let stdin = io::stdin();
    stdin.read_line(s).unwrap();
}

pub fn error_message(message: String) {
    println!("error : {}", message);
}

pub fn print_error() {
    println!("Something wrong with this console");
}
