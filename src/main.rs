mod fif;
mod tests;

use std::path::PathBuf;

use clap::Parser;

use crate::fif::*;

#[derive(Parser)]
struct CliArgs {
    pattern: String,

    path: Option<String>,

    #[cfg(feature = "regex")]
    #[clap(short = 'r', help = "Treat pattern as regex")]
    is_regex: bool,
    #[clap(short = 'i', help = "Do case insensitive pattern matching")]
    is_case_insensitive: bool,
}

#[cfg(feature = "regex")]
fn make_fif_pattern(args: &CliArgs) -> fif::Pattern {
    if args.is_regex {
        fif::Pattern::Regex(args.pattern.clone())
    } else {
        fif::Pattern::Text(args.pattern.clone())
    }
}
#[cfg(not(feature = "regex"))]
fn make_fif_pattern(args: &CliArgs) -> fif::Pattern {
    fif::Pattern::Text(args.pattern.clone())
}
impl From<CliArgs> for fif::Configuration {
    fn from(args: CliArgs) -> Self {
        Configuration {
            case_insensitive: args.is_case_insensitive,
            pattern: make_fif_pattern(&args),
        }
    }
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let root_path = args.path.clone();
    let root_path: PathBuf = root_path.unwrap_or_else(|| ".".to_string()).into();

    let fif_config: fif::Configuration = args.into();
    find_in_files(&root_path, fif_config, print_matching_lines).await;
}

fn print_matching_lines(file_name: String, matchh: fif::Match) {
    println!("{}:{} => {}", &file_name, matchh.row, &matchh.line);
}
