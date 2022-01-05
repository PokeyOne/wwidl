use std::path::PathBuf;
use crate::Config;

pub fn execute(path: &PathBuf, mut config: Config, message: Option<String>) {
    let path_str = match path.to_str() {
        Some(path_str) => path_str,
        None => {
            eprintln!("{}", "Path is not valid UTF-8");
            return;
        }
    };
    let message = match message {
        Some(message) => message,
        None => {
            println!("Please enter a note to save.");
            // read line from stdin
            let mut message = String::new();
            std::io::stdin().read_line(&mut message).unwrap();
            message.trim().to_string()
        }
    };

    config.put_note(path_str, message);
}