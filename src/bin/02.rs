use std::collections::HashSet;

use chumsky::prelude::*;
use chumsky::text::{digits, whitespace};
use num_integer::Integer;
advent_of_code::solution!(2);

fn parse(input: &str) -> Option<Vec<(u64, u64)>> {
    let num = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };
    let nums = num().then_ignore(just('-')).then(num());
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

fn add_invalid_ids_in_range(
    (mut min, mut max): (u64, u64),
    n_repeats: u32,
    invalid_ids: &mut HashSet<u64>,
) {
    let make_prefix = |val: u64, repeat_digits| val / (10_u64.pow(repeat_digits * (n_repeats - 1)));
    let make_val = |prefix, factor: u64| {
        let mut val = 0;
        for _ in 0..n_repeats {
            val = val * factor + prefix;
        }
        val
    };

    let mut min_digits = min.checked_ilog10().unwrap_or(0) + 1;
    // edge case: if min has too few digits, go up to the next multiple in digits
    if !min_digits.is_multiple_of(n_repeats) {
        min_digits = min_digits.next_multiple_of(n_repeats);
        min = 10_u64.pow(min_digits - 1);
    }
    let min_repeat_digits = min_digits / n_repeats;
    let mut min_factor = 10_u64.pow(min_repeat_digits);
    let mut min_prefix = make_prefix(min, min_repeat_digits);
    // edge case: if min_prefix repeated is smaller than min, increment min_prefix
    if make_val(min_prefix, min_factor) < min {
        min_prefix += 1;
    }

    let mut max_digits = max.checked_ilog10().unwrap_or(0) + 1;
    // edge case: if max has too many digits, go to previous multiple in digits
    if !max_digits.is_multiple_of(n_repeats) {
        max_digits = max_digits.prev_multiple_of(&n_repeats);
        max = 10_u64.pow(max_digits) - 1;
    }
    let max_repeat_digits = max_digits / n_repeats;
    let max_factor = 10_u64.pow(max_repeat_digits);
    let mut max_prefix = make_prefix(max, max_repeat_digits);
    // edge case: if max_prefix is greater than max, decrement max_prefix
    if make_val(max_prefix, max_factor) > max {
        max_prefix -= 1;
    }

    let mut cur_max_prefix = (min_factor - 1).min(max_prefix);

    while min_prefix <= max_prefix {
        for prefix in min_prefix..=cur_max_prefix {
            let value = make_val(prefix, min_factor);
            invalid_ids.insert(value);
        }
        min_prefix = min_factor;
        min_factor += 1;
        cur_max_prefix = (min_factor - 1).min(max_prefix);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let id_ranges = parse(input);
    let mut ids = HashSet::new();

    let mut total = 0;
    if let Some(ref ranges) = id_ranges {
        for &range in ranges {
            add_invalid_ids_in_range(range, 2, &mut ids);
            total += ids.iter().sum::<u64>();
            ids.clear();
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let id_ranges = parse(input);
    let mut ids = HashSet::new();
    id_ranges.map(|ranges| {
        ranges
            .iter()
            .map(|(min, max)| {
                let max_digits = max.checked_ilog10().unwrap_or(0) + 1;
                for n_repeats in 2..=max_digits {
                    add_invalid_ids_in_range((*min, *max), n_repeats, &mut ids);
                }
                let result = ids.iter().sum::<u64>();
                ids.clear();
                result
            })
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
