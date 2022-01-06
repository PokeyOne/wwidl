use std::path::PathBuf;
use crate::Config;

/// Remove a certain number of notes about a directory. Will also remove the
/// directory entry if there are no more notes left.
pub fn execute(path: &PathBuf, mut config: Config, mut count: usize) {
    // TODO: Handle this error gracefully. Just print an error and exit if it fails
    //       essentially telling the user that the file doesn't exist.
    let canon_path = path.canonicalize().unwrap();
    let path_str = match canon_path.to_str() {
        Some(path_str) => path_str,
        None => {
            eprintln!("{}", "Path is not valid UTF-8");
            return;
        }
    };

    let removed_notes = config.remove_notes(path_str, count);

    if removed_notes.len() == 0 {
        println!("No notes found for {}", path_str);
    } else if removed_notes.len() == 1 {
        println!("Removed 1 note for {}", path_str);
    } else {
        println!("Removed {} notes for {}", removed_notes.len(), path_str);
    }

    config.save();
}
