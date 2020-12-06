use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    let day_one_input = read_file_to_usize_vec("input/day_1_input").unwrap();
    println!("Day one part one answer: {}", day_one_part_one(&day_one_input).unwrap());
    println!("Day one part two answer: {}", day_one_part_two(&day_one_input).unwrap());
}