use std::error::Error;

use std::{fs, io};
use std::io::BufRead;
use std::borrow::Cow;
use std::path::PathBuf;   

pub struct FifConfiguration {
    case_sensitive: bool,
}

impl Default for FifConfiguration {
    fn default() -> Self {
        FifConfiguration { 
            case_sensitive: true
        }
    }
}

pub fn find_in_files(directory_name: &str, pattern: &str) {
    let folder = fs::read_dir(directory_name).expect("could not open dir");
    for entry in folder.filter(|f| f.is_ok())
        .map(|f|f.unwrap())
    {
        let dir_path = entry.path();
        let dir_name = dir_path.as_os_str();
        let dir_name = dir_name.to_string_lossy();
        let metadata = entry.metadata().expect("Failed to open metadata ");
        if metadata.is_dir() {
            find_in_files(&dir_name, pattern);
        } else if let Err(e) = find_in_file(&dir_path, pattern) {
            eprintln!("Error while analyzing {}: {}", entry.file_name().to_str().unwrap(), e);
        }

    }
}

fn entry_path_to_str(entry_path: &PathBuf) -> Cow<str> {
    let file_name = entry_path.as_os_str();
    file_name.to_string_lossy()
}

pub fn find_in_file(entry: &PathBuf, pattern: &str) -> Result<(), Box<dyn Error>> {
    let file_name = entry_path_to_str(&entry);
    let file = fs::File::open(entry)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines()
        .filter_map(|line| line.ok());
    let matches = find_in_lines(lines, pattern);
    print_matching_lines(file_name.as_ref(), matches);
    Ok(())
}

pub fn find_in_lines<T, U>(lines: T, pattern: &str) -> Vec<String>
    where U: ToString, 
        T: Iterator<Item = U> {
    lines
        .map(|line| line.to_string())
        .filter(|line| line.contains(&pattern))
        .collect()
}

fn print_matching_lines(file_name: &str, matches: Vec<String>) {
    for matching in matches {
        println!("{} => {}", file_name, matching);
    }
}