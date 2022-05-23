use std::collections::HashMap;
use std::error::Error;

use std::{fs, io};
use std::io::BufRead;
use std::path::PathBuf;

pub type Matches = dyn Iterator<Item = Match>;
pub enum Pattern {
    Text(String),
    #[cfg(feature = "regex")]
    Regex(String)
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
            #[cfg(feature = "regex")]
            Self::Regex(r) => Self::Regex(r.clone()),
        }
    }
}

impl Pattern {
    pub fn text_from_string<T: ToString + ?Sized>(string: &T) -> Self {
        Pattern::Text(string.to_string())
    }

    #[cfg(feature = "regex")]
    pub fn regex_from_string<T: ToString>(string: T) -> Self {
        Pattern::Regex(string.to_string())
    }

    pub fn into_matching_function(self, case_insensitive: bool) -> Box<dyn Fn(&str) -> bool> {
        match self {
            Pattern::Text(t) => 
            {
                if case_insensitive {
                    Box::new(move |line: &str| line.to_lowercase().contains(&t)) 
                } else {
                    Box::new(move |line: &str| line.contains(&t))
                }
                
            },
            #[cfg(feature = "regex")]
            Pattern::Regex(regex) => {
                let mut regex_builder = regex::RegexBuilder::new(&regex);
                regex_builder.case_insensitive(case_insensitive);

                let regex = regex_builder.build().expect("Failed to build regex");
                Box::new(move |line: &str| regex.is_match(line))
            }
        }
    }
}

pub struct Configuration {
    pub case_insensitive: bool,
    pub pattern: Pattern,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { 
            case_insensitive: false,
            pattern: Default::default(),
        }
    }
}

impl Configuration {
    pub fn default_from_pattern<T: ToString + ?Sized>(pattern: &T) -> Configuration {
        Configuration { 
            pattern: Pattern::text_from_string(pattern),
            ..Default::default()
        }
    }

    pub fn clone_pattern(&self) -> Pattern {
        self.pattern.clone()
    }
}

pub struct Match {
    pub row: usize,
    pub line: String,
}

pub fn find_in_files(directory_name: &PathBuf, configuration: &Configuration) -> HashMap<String, Box<Matches>> {
    let folder = fs::read_dir(directory_name).expect("could not open dir");
    let mut matches_in_files: HashMap<String, Box<Matches>> = HashMap::new();
    for entry in folder.filter(|f| f.is_ok())
        .map(|f|f.unwrap())
    {
        let dir_path = entry.path();
        let metadata = entry.metadata().expect("Failed to open metadata ");
        if metadata.is_dir() {
            matches_in_files.extend(find_in_files(&dir_path, &configuration));
        } else {
            match find_in_file(&dir_path, &configuration) {
                Ok(lines) => { 
                    let file_name = dir_path.as_os_str();
                    let file_name = file_name.to_string_lossy().to_string();
                    matches_in_files.insert(file_name, lines);
                },
                Err(e) => eprintln!("Error while analyzing {}: {}", entry.file_name().to_str().unwrap(), e)
            };
        }
    }
    matches_in_files
}

pub fn find_in_file(entry: &PathBuf, configuration: &Configuration) -> Result<Box<Matches>, Box<dyn Error>> {
    let file = fs::File::open(entry)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines()
        .map(|line| line.unwrap_or_default());
    let matches = find_in_lines(lines, &configuration);
    Ok(Box::new(matches))
}

pub fn find_in_lines<T, U>(lines: T, configuration: &Configuration) -> impl Iterator<Item = Match>
    where U: ToString, 
        T: Iterator<Item = U> {
    let match_predicate  = construct_filtering_predicate(configuration);
    lines
        .map(|line| line.to_string())
        .enumerate()
        .filter(move |(_, line) | match_predicate(&line))
        .map(|(row, line)| {
            Match {
                row: row + 1,
                line
            }
        })
}

fn construct_filtering_predicate(configuration: &Configuration) -> Box<dyn Fn(&str) -> bool> {
    let pattern_clone = configuration.clone_pattern();
    pattern_clone.into_matching_function(configuration.case_insensitive)
}