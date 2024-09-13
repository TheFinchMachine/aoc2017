use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

pub fn string_from_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;

    //read into string?
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

pub fn sanitize_numeric(s: &str) -> String{
    let mut new_s = String::new();
    for c in s.chars() {
        if c.is_ascii_digit(){
            new_s.push(c);
        }
    }
    new_s
}

pub fn sanitize_same_next_only(s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use crate::sanitize_same_next_only;

    #[test]
    fn sanitize_same_next_only_unit() {
        let inputs = vec!["1122", "1111", "1234", "91212129"];
        let answers = vec!["12", "1111", "", "9"];
        assert_eq!(inputs.len(), answers.len(), "Number of inputs for test, {}, is not equal to the number of answers, {}", inputs.len(), answers.len());

        for (i, s) in inputs.iter().enumerate() {
            let a = sanitize_same_next_only(*s);
            assert_eq!(answers[i], &a, "Failed to process string {}. Expected result was {} but actuall result was {}", *s, answers[i], &a);
        }
    }
}