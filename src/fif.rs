use std::error::Error;

use std::{fs, io};
use std::io::BufRead;
use std::borrow::Cow;
use std::path::PathBuf;   

pub enum Pattern {
    Text(String)
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Text(String::default())
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        match self {
            Self::Text(text) => Self::Text(text.clone()),
        }
    }
}

impl Pattern {
    pub fn from_string<T: ToString + ?Sized>(string: &T) -> Self {
        Pattern::Text(string.to_string())
    }
}

pub struct Configuration {
    pub case_sensitive: bool,
    pub pattern: Pattern,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { 
            case_sensitive: true,
            pattern: Default::default()
        }
    }
}

impl Configuration {
    pub fn default_from_pattern<T: ToString + ?Sized>(pattern: &T) -> Configuration {
        Configuration { 
            pattern: Pattern::from_string(pattern),
            ..Default::default()
        }
    }

    pub fn clone_pattern(&self) -> Pattern {
        self.pattern.clone()
    }
}

pub fn find_in_files(directory_name: &str, configuration: &Configuration) {
    let folder = fs::read_dir(directory_name).expect("could not open dir");
    for entry in folder.filter(|f| f.is_ok())
        .map(|f|f.unwrap())
    {
        let dir_path = entry.path();
        let dir_name = dir_path.as_os_str();
        let dir_name = dir_name.to_string_lossy();
        let metadata = entry.metadata().expect("Failed to open metadata ");
        if metadata.is_dir() {
            find_in_files(&dir_name, &configuration);
        } else if let Err(e) = find_in_file(&dir_path, &configuration) {
            eprintln!("Error while analyzing {}: {}", entry.file_name().to_str().unwrap(), e);
        }

    }
}

fn entry_path_to_str(entry_path: &PathBuf) -> Cow<str> {
    let file_name = entry_path.as_os_str();
    file_name.to_string_lossy()
}

pub fn find_in_file(entry: &PathBuf, configuration: &Configuration) -> Result<(), Box<dyn Error>> {
    let file_name = entry_path_to_str(&entry);
    let file = fs::File::open(entry)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines()
        .filter_map(|line| line.ok());
    let matches = find_in_lines(lines, &configuration);
    print_matching_lines(file_name.as_ref(), matches);
    Ok(())
}

pub fn find_in_lines<T, U>(lines: T, configuration: &Configuration) -> Vec<String>
    where U: ToString, 
        T: Iterator<Item = U> {        
    let pattern_clone = configuration.clone_pattern();
    let match_function: Box<dyn Fn(&str) -> bool> = match pattern_clone {
        Pattern::Text(t) => Box::new(move |line: &str| line.contains(&t)),
    };

    let filter_pred : Box<dyn Fn(&String) -> bool> = if configuration.case_sensitive {
        Box::new(move |line: &String| match_function(&line))
    } else {
        Box::new(move |line: &String| match_function(&line.to_lowercase()))
    };
    
    lines
        .map(|line| line.to_string())
        .filter(filter_pred)
        .collect()
}

fn print_matching_lines(file_name: &str, matches: Vec<String>) {
    for matching in matches {
        println!("{} => {}", file_name, matching);
    }
}