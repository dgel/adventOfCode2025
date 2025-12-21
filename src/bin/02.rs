use std::io::Write;
use std::ops::RangeInclusive;

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

fn count_invalid_ids_in_range((mut min, mut max): (u64, u64), n_repeats: u32) -> u64 {
    println!("min: {}, max: {}, n_repeats: {}", min, max, n_repeats);
    let make_prefix = |val: u64, repeat_digits| val / (10_u64.pow(repeat_digits * (n_repeats - 1)));
    let make_val = |prefix, factor: u64| {
        // println!(
        //     "prefix: {}, factor: {}, n_repeats: {}",
        //     prefix, factor, n_repeats
        // );
        let mut val = 0;
        for _ in 0..n_repeats {
            val = val * factor + prefix;
        }
        val
    };

    let mut min_digits = min.checked_ilog10().unwrap_or(0) + 1;
    // println!("min_digits: {}", min_digits);
    // edge case: if min has too few digits, go up to the next multiple in digits
    if !min_digits.is_multiple_of(n_repeats) {
        // print!("Changing min from {} ", min);
        min_digits = min_digits.next_multiple_of(n_repeats);
        min = 10_u64.pow(min_digits - 1);
        // println!("to {} ", min);
    }
    let min_repeat_digits = min_digits / n_repeats;
    // println!("min_repeat_digits: {}", min_repeat_digits);
    let mut min_factor = 10_u64.pow(min_repeat_digits);
    let mut min_prefix = make_prefix(min, min_repeat_digits);
    // println!(
    //     "min_prefix before: {}, make_val: {}",
    //     min_prefix,
    //     make_val(min_prefix, min_factor, n_repeats)
    // );
    // edge case: if min_prefix repeated is smaller than min, increment min_prefix
    if make_val(min_prefix, min_factor) < min {
        min_prefix += 1;
    }
    // println!("min_prefix: {}", min_prefix);

    let mut max_digits = max.checked_ilog10().unwrap_or(0) + 1;
    // println!("max_digits: {}", max_digits);
    // edge case: if max has too many digits, go to previous multiple in digits
    if !max_digits.is_multiple_of(n_repeats) {
        // print!("Changing max from {} ", max);
        max_digits = max_digits.prev_multiple_of(&n_repeats);
        max = 10_u64.pow(max_digits) - 1;
        // println!("to {} ", max);
    }
    let max_repeat_digits = max_digits / n_repeats;
    let max_factor = 10_u64.pow(max_repeat_digits);
    let mut max_prefix = make_prefix(max, max_repeat_digits);
    // edge case: if max_prefix is greater than max, decrement max_prefix
    if make_val(max_prefix, max_factor) > max {
        max_prefix -= 1;
    }
    // println!("max_prefix: {}", max_prefix);

    let mut sum = 0;
    let mut cur_max_prefix = (min_factor - 1).min(max_prefix);
    // println!("cur_max_prefix: {}", cur_max_prefix);

    while min_prefix <= max_prefix {
        for prefix in min_prefix..=cur_max_prefix {
            let value = make_val(prefix, min_factor);
            println!("Found value: {}", value);
            sum += value;
        }
        min_prefix = min_factor;
        min_factor += 1;
        cur_max_prefix = (min_factor - 1).min(max_prefix);
    }
    sum

    // edge cases:
    //    min is not divisible by n:
    //        adjust min until divisible
    //    max is not divisible by n:
    //        adjust max until divisible
    //    min prefix > multiplied prefix:
    //       adjust min prefix?
    //    max prefix < multiplied prefix
    //       adjust max prefix?

    // set cur_max
    // while cur_max < max:
    //     set min_prefix
    //     set max_prefix
    //     add sum of values
    //

    // let mut sum = 0;
    // // println!("cur_max: {}", cur_max);
    // while cur_max < max {
    //     let mut min_prefix = min / (10_u64.pow(digits / 2));
    //     if min < (min_prefix * 10_u64.pow(digits / 2) + min_prefix) {
    //         min_prefix += 1;
    //     }
    //     let max_prefix = cur_max / 10_u64.pow(digits / 2);
    //     for i in min_prefix..=max_prefix {
    //         sum += i * 10_u64.pow(digits / 2) + i;
    //     }
    //     // println!("In loop, adding to sum: {}", addition);

    //     digits += 2;
    //     min = 10_u64.pow(digits - 1);
    //     cur_max = 10_u64.pow(digits) - 1;
    // }

    // // println!("beyond loop, min: {}, max: {}", min, max);
    // if min <= max {
    //     let mut count_max = max / 10_u64.pow(digits / 2);

    //     let new_max = count_max * 10_u64.pow(digits / 2) + count_max;
    //     if new_max > max {
    //         count_max -= 1;
    //     }
    //     count += count_max - (min / 10_u64.pow(digits / 2)) + 1;
    // }

    // count
}

fn is_invalid_id(id: u64, buffer: &mut Vec<u8>) -> bool {
    let num_digits = id.checked_ilog10().unwrap_or(0) + 1;
    if !num_digits.is_multiple_of(2) {
        return false;
    }
    write!(buffer, "{}", id).unwrap();
    let middle = num_digits / 2;
    let (first, second) = buffer.split_at(middle as usize);
    let result = first == second;
    buffer.clear();
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let id_ranges = parse(input);
    // let mut buf = Vec::new();

    let mut total = 0;
    if let Some(ref ranges) = id_ranges {
        for &range in ranges {
            let result = count_invalid_ids_in_range(range, 2);
            total += result;
            // println!("count for range {} - {}: {}", range.0, range.1, result);
        }
    }
    println!("Alternative result: {}", total);

    // id_ranges.map(|ranges| {
    //     ranges
    //         .iter()
    //         .flat_map(|&(min, max)| min..=max)
    //         .filter(|&id| is_invalid_id(id, &mut buf))
    //         .sum()
    // })
    Some(total)
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
            .map(|(min, max)| {
                let max_digits = max.checked_ilog10().unwrap_or(0) + 1;
                (2..=max_digits)
                    .map(|n_repeats| count_invalid_ids_in_range((*min, *max), n_repeats))
                    .sum::<u64>()
            })
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(1227775554));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
