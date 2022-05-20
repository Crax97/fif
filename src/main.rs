mod fif;
mod tests;

use crate::fif::*;

fn main() {
    if std::env::args().len() < 3 {
        let program_name = std::env::args().nth(0).expect("No program name! wtf?");
        eprintln!("usage: {0} directory pattern", program_name);
    }    

    let directory_name = std::env::args().nth(1).expect("No directory!");
    let pattern = std::env::args().nth(2).expect("No pattern!");

    find_in_files(directory_name.as_ref(), pattern.as_ref());
}