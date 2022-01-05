use std::path::PathBuf;
use crate::Config;

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
        Some(_data) => {
            todo!()
        }
        None => {
            println!("No repo data found");
        }
    }
}