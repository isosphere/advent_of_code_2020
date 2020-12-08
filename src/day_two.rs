use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

pub fn day_two_file_reader(file: &str) -> Result<Vec<(usize, usize, String, String)>, String> {
    let file = match File::open(file) {
        Ok(io) => io,
        Err(err) => return Err(err.to_string())
    };
    
    let reader = BufReader::new(file);

    let mut output = Vec::new();
    let re = Regex::new(r"(?P<minimum>\d+)-(?P<maximum>\d+)\s(?P<character>\w):\s(?P<password>\w+)").unwrap();

    for line in reader.lines() {
        match line {
            Ok(v) => {
                match re.captures(&v) {
                    Some(capture) => output.push((
                        capture.name("minimum").unwrap().as_str().parse::<usize>().unwrap(),
                        capture.name("maximum").unwrap().as_str().parse::<usize>().unwrap(),
                        capture.name("character").unwrap().as_str().to_owned(), 
                        capture.name("password").unwrap().as_str().to_owned()
                    )),
                    None => return Err("No match on line".to_owned())
                }
            },
            Err(err) => return Err(err.to_string())
        }
    }

    Ok(output)
}

pub fn day_two_part_one_password_verify(minimum: usize, maximum: usize, character: &str, password: &str) -> bool {
    let counter = password.chars().fold(0, |acc, c| match character.starts_with(c) {true => acc + 1, false => acc} );

    counter <= maximum && counter >= minimum
}

#[test]
fn test_day_two_part_one_password_verify() {
    assert!(day_two_part_one_password_verify(1, 3, "a", "abcde"));
    assert!(!day_two_part_one_password_verify(1, 3, "b", "cdefg"));
    assert!(day_two_part_one_password_verify(2, 9, "c", "ccccccccc"));
}

pub fn day_two_part_two_password_verify(index_a: usize, index_b: usize, character: &str, password: &str) -> bool {
    let char_array: Vec<(usize, char)> = password.char_indices().collect();
    assert!(index_a > 0 && index_a <= password.len(), "index_a out of bounds");
    assert!(index_b > 0 && index_b <= password.len(), "index_b out of bounds");
    character.starts_with(char_array[index_a - 1].1) ^ character.starts_with(char_array[index_b - 1].1)
}

#[test]
fn test_day_two_part_two_password_verify() {
    assert!(day_two_part_two_password_verify(1, 3, "a", "abcde"));
    assert!(!day_two_part_two_password_verify(1, 3, "b", "cdefg"));
    assert!(!day_two_part_two_password_verify(2, 9, "c", "ccccccccc"));
}