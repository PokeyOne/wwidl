use std::path::PathBuf;
use crate::Config;

// TODO: Show all argument
pub fn execute(path: &PathBuf, config: Config) {
    let canon_path = path.canonicalize().unwrap();
    let path_str = match canon_path.to_str() {
        Some(path_str) => path_str,
        None => {
            eprintln!("{}", "Path is not valid UTF-8");
            return;
        }
    };
    match config.repo_data(path_str) {
        Some(data) => {
            let last_message = match data.last_message() {
                Some(last_message) => last_message,
                None => {
                    println!("No repo data found");
                    return
                }
            };
            println!("{}:\n\t{}", last_message.date(), last_message.message());
        }
        None => {
            println!("No repo data found");
        }
    }
}