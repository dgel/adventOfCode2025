use std::ops::RangeInclusive;

use chumsky::prelude::*;
use chumsky::text::{digits, whitespace};
advent_of_code::solution!(2);

fn parse(input: &str) -> Option<Vec<RangeInclusive<u64>>> {
    let num = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };
    let nums = num()
        .then_ignore(just('-'))
        .then(num())
        .map(|(start, end)| start..=end);
    let parser = nums
        .separated_by(just(','))
        .collect()
        .then_ignore(whitespace());
    match parser.parse(input).into_result() {
        Ok(result) => Some(result),
        Err(errors) => {
            for error in errors {
                println!("Failed to parse input: {}", error);
            }
            None
        }
    }
}

fn is_invalid_id(id: u64) -> bool {
    let num_digits = id.checked_ilog10().unwrap_or(0) + 1;
    if !num_digits.is_multiple_of(2) {
        return false;
    }
    let formatted = id.to_string().into_bytes();
    let middle = num_digits / 2;
    let (first, second) = formatted.as_slice().split_at(middle as usize);
    first == second
}

pub fn part_one(input: &str) -> Option<u64> {
    let id_ranges = parse(input);
    id_ranges.map(|ranges| {
        ranges
            .iter()
            .flat_map(|range| range.clone())
            .filter(|&id| is_invalid_id(id))
            .sum()
    })
}

fn is_invalid_id_2(id: u64) -> bool {
    let formatted = id.to_string().into_bytes();
    for pattern_length in 1..=(formatted.len() / 2) {
        if !formatted.len().is_multiple_of(pattern_length) {
            continue;
        }

        let num_copies = formatted.len() / pattern_length;
        let first = &formatted[0..pattern_length];

        let mut is_valid = false;
        for nth_copy in 1..num_copies {
            let nth_copy_index = nth_copy * pattern_length;
            let copy = &formatted[nth_copy_index..nth_copy_index + pattern_length];
            if first != copy {
                is_valid = true;
                break;
            }
        }
        if !is_valid {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let id_ranges = parse(input);
    id_ranges.map(|ranges| {
        ranges
            .iter()
            .flat_map(|range| range.clone())
            .filter(|&id| is_invalid_id_2(id))
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
