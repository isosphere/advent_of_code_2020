use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn day_one_part_one(expenses: &[usize]) -> Result<usize, ()> {
    for a in expenses {
        for b in expenses {
            match a + b {
                2020 => return Ok(a * b),
                _ => continue
            }
        }
    }
    Err(())
}

#[test]
fn test_day_one_part_one() {
    let example = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(day_one_part_one(&example), Ok(514579));
}

fn day_one_part_two(expenses: &[usize]) -> Result<usize, ()> {
    for a in expenses {
        for b in expenses {
            for c in expenses {
                match a + b + c {
                    2020 => return Ok(a * b * c),
                    _ => continue
                }
            }
        }
    }
    Err(())
}

#[test]
fn test_day_one_part_two() {
    let example = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(day_one_part_two(&example), Ok(241861950));
}

fn read_file_to_usize_vec(file: &str) -> Result<Vec<usize>, String> {
    let file = match File::open(file) {
        Ok(io) => io,
        Err(err) => return Err(err.to_string())
    };
    
    let reader = BufReader::new(file);

    let mut output = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(v) => {
                match v.parse::<usize>() {
                    Ok(value) => output.push(value),
                    Err(err) => return Err(err.to_string())
                }
            },
            Err(err) => return Err(err.to_string())
        }
    }

    Ok(output)
}

fn day_two_file_reader(file: &str) -> Result<Vec<(usize, usize, String, String)>, String> {
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

fn day_two_part_one_password_verify(minimum: usize, maximum: usize, character: &str, password: &str) -> bool {
    let counter = password.chars().fold(0, |acc, c| match character.starts_with(c) {true => acc + 1, false => acc} );

    counter <= maximum && counter >= minimum
}

#[test]
fn test_day_two_part_one_password_verify() {
    assert!(day_two_part_one_password_verify(1, 3, "a", "abcde"));
    assert!(!day_two_part_one_password_verify(1, 3, "b", "cdefg"));
    assert!(day_two_part_one_password_verify(2, 9, "c", "ccccccccc"));
}

fn day_two_part_two_password_verify(index_a: usize, index_b: usize, character: &str, password: &str) -> bool {
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


fn main() {
    let day_one_input = read_file_to_usize_vec("input/day_1_input").unwrap();
    println!("Day one part one answer: {}", day_one_part_one(&day_one_input).unwrap());
    println!("Day one part two answer: {}", day_one_part_two(&day_one_input).unwrap());

    let day_two_input = day_two_file_reader("input/day_2_input").unwrap();
    let day_two_part_one = day_two_input.iter().fold(
        0, |acc, (minimum, maximum, character, password)| match day_two_part_one_password_verify(*minimum, *maximum, character, password) {
            true => acc + 1,
            false => acc
        }
    );
    println!("Day two part one answer: {}", day_two_part_one);
    let day_two_part_two = day_two_input.iter().fold(
        0, |acc, (index_a, index_b, character, password)| match day_two_part_two_password_verify(*index_a, *index_b, character, password) {
            true => acc + 1,
            false => acc
        }
    );
    println!("Day two part two answer: {}", day_two_part_two);
}