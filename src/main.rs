use std::fs::File;
use std::io::{BufRead, BufReader};

mod day_one;
use day_one::{day_one_part_one, day_one_part_two};

mod day_two;
use day_two::{day_two_file_reader, day_two_part_one_password_verify, day_two_part_two_password_verify};

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