pub fn day_one_part_one(expenses: &[usize]) -> Result<usize, ()> {
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

pub fn day_one_part_two(expenses: &[usize]) -> Result<usize, ()> {
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