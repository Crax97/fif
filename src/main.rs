mod fif;
mod tests;

use std::{path::PathBuf, collections::HashMap};

use clap::Parser;

use crate::fif::*;

#[derive(Parser)]
struct CliArgs {
    pattern: String,

    path: Option<String>,
    
    #[cfg(feature="regex")]
    #[clap(short='r',help="Treat pattern as regex")]
    is_regex: bool,
    #[clap(short='i', help="Do case insensitive pattern matching")]
    is_case_insensitive: bool,
}

#[cfg(feature="regex")]
fn make_fif_pattern(args: &CliArgs) -> fif::Pattern {
    if args.is_regex {
        fif::Pattern::Regex(args.pattern.clone())
    } else {
        fif::Pattern::Text(args.pattern.clone())
    }
}
#[cfg(not(feature="regex"))]
fn make_fif_pattern(args: &CliArgs) -> fif::Pattern {
    fif::Pattern::Text(args.pattern.clone())
}
impl Into<fif::Configuration> for CliArgs {
    fn into(self) -> fif::Configuration {
        Configuration { 
            case_insensitive: self.is_case_insensitive,
            pattern: make_fif_pattern(&self)
        }
    }
}


fn main() {
    let args = CliArgs::parse();
    let root_path = args.path.clone();
    let root_path : PathBuf = root_path.unwrap_or(".".to_string()).clone().into();

    let fif_config : fif::Configuration = args.into();
    let matching_results = find_in_files(&root_path, &fif_config);
    print_matching_lines(matching_results);
}

fn print_matching_lines(matching_results: HashMap<String, Vec<fif::Match>>) {
    
    for (file, matchs) in matching_results {
        for matchh in matchs.iter() {
            println!("{}:{} => {}", &file, matchh.row, &matchh.line);
        }
    }
}