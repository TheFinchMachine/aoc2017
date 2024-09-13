use std::path::Path;
use aoc2017::*;

fn main() {
    //open file
    let path = Path::new("res/puzzle1.txt");
    let display = path.display();

    let mut in_day1 = string_from_file(path).unwrap_or_else(|why| {
        panic!("couldn't open {}: {}", display, why)
    });

    //sanitize string to numeric characters
    in_day1 = sanitize_numeric(&in_day1);

    //remove non-matching numbers from string (remember the string loops!)
    in_day1 = sanitize_same_next_only(&in_day1);

    //sum remaining string
    let sum = sum_chars_in_string(&in_day1);

    println!("in_day1: {:?}", in_day1);
    println!("sum of in_day1: {}", sum);
}
