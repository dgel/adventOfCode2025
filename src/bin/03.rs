use chumsky::prelude::*;
use chumsky::text::{digits, whitespace};

advent_of_code::solution!(3);

fn parse(input: &str) -> Option<Vec<Vec<u8>>> {
    let nums = digits::<&str, extra::Err<Rich<char>>>(10)
        .to_slice()
        .map(|chs| chs.bytes().map(|ch| ch - b'0').collect());
    let rows = nums.separated_by(whitespace()).allow_trailing().collect();
    match rows.parse(input).into_result() {
        Ok(result) => Some(result),
        Err(errors) => {
            for error in errors {
                println!("Failed to parse input: {}", error);
            }
            None
        }
    }
}

fn get_highest_two(row: &[u8]) -> u32 {
    if row.is_empty() {
        return 0;
    }

    let mut digit1 = 0;
    let mut digit2 = 0;

    for &digit in &row[0..row.len() - 1] {
        if digit > digit1 {
            digit1 = digit;
            digit2 = 0;
        } else if digit > digit2 {
            digit2 = digit;
        }
    }

    let last = *row.last().unwrap(); // valid due to early exit above
    if last > digit2 {
        digit2 = last;
    }
    digit1 as u32 * 10 + digit2 as u32
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse(input);

    data.map(|rows| rows.iter().map(|row| get_highest_two(row)).sum::<u32>() as u64)
}

fn get_highest_twelve(row: &[u8]) -> u64 {
    let mut chosen_digits = Vec::new();

    for index in 0..row.len() {
        let remaining = row.len() - index;
        let skip_n = 12_usize.saturating_sub(remaining);

        let digit = row[index];
        let mut pushed = false;
        for j in skip_n..chosen_digits.len() {
            if digit > chosen_digits[j] {
                chosen_digits.resize(j, 0);
                chosen_digits.push(digit);
                pushed = true;
                break;
            }
        }
        if !pushed && chosen_digits.len() < 12 {
            chosen_digits.push(digit);
        }
    }
    chosen_digits
        .iter()
        .fold(0, |acc, &val| acc * 10 + val as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse(input);

    data.map(|rows| rows.iter().map(|row| get_highest_twelve(row)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
