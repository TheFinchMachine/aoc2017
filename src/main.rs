use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

fn string_from_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;

    //read into string?
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

fn sanitize_numeric(s: &str) -> String{
    let mut new_s = String::new();
    for c in s.chars() {
        if c.is_ascii_digit(){
            new_s.push(c);
        }
    }
    new_s
}

fn sanitize_same_next_only(s: &str) -> String {
    let mut new_s = String::new();
    if s.len() <= 1 {
        return new_s;
    }

    let chars: Vec<char> = s.chars().collect();
    for w in chars.windows(2) {
        if w[0] == w[1] {
            new_s.push(w[0]);
        }
    }

    let last_char = chars.last().unwrap();
    if last_char == chars.first().unwrap() {
        new_s.push(*last_char);
    }

    new_s
}

fn main() {
    //open file
    let path = Path::new("res/puzzle1.txt");
    let display = path.display();

    let mut in_day1 = string_from_file(path).unwrap_or_else(|why| {
        panic!("couldn't open {}: {}", display, why)
    });

    //sanitize string to numeric characters
    in_day1 = sanitize_numeric(&in_day1);

    in_day1 = sanitize_same_next_only(&in_day1);

    //remove non-matching numbers from string (remember the string loops!)
    //sum remaining string

    println!("in_day1: {:?}", in_day1);
}
