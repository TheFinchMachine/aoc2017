use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::fmt;
use crate::*;

// Define the custom error type
#[derive(Debug)]
pub enum SanitizeError {
    StringTooShort,
}

// Implement Display for SanitizeError to provide a human-readable description
impl fmt::Display for SanitizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SanitizeError::StringTooShort => write!(f, "string too short"),
        }
    }
}

// Optionally implement the Error trait for compatibility with other error handling libraries
impl std::error::Error for SanitizeError {}

pub fn sanitize_numeric(s: &str) -> String{
    let mut new_s = String::new();
    for c in s.chars() {
        if c.is_ascii_digit(){
            new_s.push(c);
        }
    }
    new_s
}

pub fn sanitize_same_jump_only(s: &str) -> Result<String, SanitizeError> {
    if s.len() <= 2 {
        return Err(SanitizeError::StringTooShort);
    }

    let s_win = format!("{}{}", s, &s[0..s.len()/2]);

    //window size is larger because we need to included the element, not just the skiped elements.
    Ok(match_chars_from_window(&s_win, s.len()/2+1))
}

pub fn sanitize_same_next_only(s: &str) -> Result<String, SanitizeError> {
    if s.len() <= 2 {
        return Err(SanitizeError::StringTooShort);
    }

    let s_win = format!("{}{}", s, &s[0..1]);

    Ok(match_chars_from_window(&s_win, 2))
}

fn match_chars_from_window(s: &str, window_size: usize) -> String {
    let mut new_s = String::new();
 
    let chars: Vec<char> = s.chars().collect();
    for w in chars.windows(window_size) {
        if let (Some(first), Some(last)) = (w.first(), w.last()) {
            if first == last {
                new_s.push(*first)
            }
        }
    }

    new_s
}

pub fn sum_chars_in_string(s: &str) -> u32 {
    let mut sum = 0;
    for c in s.chars() {
        sum += c.to_digit(10).unwrap_or(0);
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

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

    #[test]
    fn sanitize_same_jump_only_unit() {
        let inputs = vec!["1212", "1221", "123425", "123123", "12131415"];
        let answers = vec!["1212", "", "22", "123123", "1111"];
        assert_eq!(inputs.len(), answers.len(), "Number of inputs for test, {}, is not equal to the number of answers, {}", inputs.len(), answers.len());

        for (i, s) in inputs.iter().enumerate() {
            let a = sanitize_same_jump_only(*s);
            assert_eq!(answers[i], &a, "Failed to process string {}. Expected result was {} but actuall result was {}", *s, answers[i], &a);
        }
    }
}

pub fn solve_puzzles() {
        //open file
        let path = Path::new("res/puzzle1.txt");
        let display = path.display();
    
        let mut in_day1 = string_from_file(path).unwrap_or_else(|why| {
            panic!("couldn't open {}: {}", display, why)
        });
    
        //sanitize string to numeric characters
        in_day1 = sanitize_numeric(&in_day1);
    
        //remove non-matching numbers from string (remember the string loops!)
        let day1_puzzle1 = sanitize_same_next_only(&in_day1).unwrap();
        let day1_puzzle2 = sanitize_same_jump_only(&in_day1).unwrap();
    
        //sum remaining string
        let sum_puzzle1 = sum_chars_in_string(&day1_puzzle1);
        let sum_puzzle2 = sum_chars_in_string(&day1_puzzle2);
    
        println!("in_day1: {:?}", in_day1);
        println!("sum of puzzle1: {}", sum_puzzle1);
        println!("sum of puzzle2: {}", sum_puzzle2);
}