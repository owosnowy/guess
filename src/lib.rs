use std:: {
    io::{
        stdin,
        stdout,
        Write,
    },
    process as proc,
};

pub fn input() -> String {
    let mut userstring = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut userstring).expect("Did not enter a correct string");
    if let Some('\n') = userstring.chars().next_back() {
        userstring.pop();
    }
    if let Some('\r') = userstring.chars().next_back() {
        userstring.pop();
    }
    if userstring.trim().is_empty() {
        eprintln!("No content was entered");
        proc::exit(0);
    }
    userstring
}