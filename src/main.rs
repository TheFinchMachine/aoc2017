use aoc2017::*;
use std::path::Path;

fn spreadsheet_to_numbers(sheet: &str) -> Vec::<Vec<i32>> {
    let rows: Vec<&str> = sheet.split("\n").collect();
    let mut row_numbers = Vec::<Vec<i32>>::new();
    for row in rows {
        let words: Vec<&str> = row.split_whitespace().collect();
        let mut numbers = Vec::<i32>::new();
        for word in words {
            let number = word.parse().unwrap();
            numbers.push(number);
        }
        row_numbers.push(numbers);
    }
    row_numbers
}

fn sum_min_max(table: Vec::<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for mut row in table {
        if !row.is_empty() {
            row.sort();
            sum += row.last().unwrap() - row.first().unwrap();
            println!("{:?}", row);
        }
    }
    sum
}

fn sum_divisable(table: Vec::<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in table {
        sum += row_find_divisible(row);
    }
    sum
}

fn row_find_divisible(row: Vec<i32>) -> i32 {
    let row_compare = &row.clone();
    for i in &row {
        for j in row_compare {
            if j != i && j % i == 0 {
                println!("{}, {}", j, i);
                return j/i
            }
        }
    }
    0 
}

fn main() {

    let path = Path::new("res/puzzle2.txt");
    let display = path.display();

    let s = string_from_file(path)
        .unwrap_or_else(|why| panic!("couldn't open {}: {}", display, why));

    let row_numbers = spreadsheet_to_numbers(&s);

    //let sum = sum_min_max(row_numbers);
    let sum = sum_divisable(row_numbers);

    println!("{}", sum);
}
