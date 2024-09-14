pub mod day1;
pub mod day2;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;

pub fn string_from_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;

    //read into string?
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}