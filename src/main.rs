use std::error::Error;
use std::fs::DirEntry;
use std::{fs, io};
use std::io::BufRead;
use std::borrow::Cow;
use std::path::PathBuf;

fn main() {
    if std::env::args().len() < 3 {
        let program_name = std::env::args().nth(0).expect("No program name! wtf?");
        eprintln!("usage: {0} directory pattern", program_name);
    }    

    let directory_name = std::env::args().nth(1).expect("No directory!");
    let pattern = std::env::args().nth(2).expect("No pattern!");

    find_in_files(directory_name.as_ref(), pattern.as_ref());
}

fn find_in_files(directory_name: &str, pattern: &str) {
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

fn find_in_file(entry: &PathBuf, pattern: &str) -> Result<(), Box<dyn Error>> {
    let file_name = entry_path_to_str(&entry);
    let file = fs::File::open(entry)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines().filter_map(|f| { f.ok() }) {
        if line.contains(&pattern) {
            println!("{} => {}", &file_name, line);
        }
    }

    Ok(())
}