use chumsky::prelude::*;
use chumsky::text::{digits, newline, whitespace};

advent_of_code::solution!(5);

struct Input {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

fn parse(input: &str) -> Option<Input> {
    let num = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };
    let range = num().then_ignore(just('-')).then(num());
    let ranges = range.separated_by(newline()).collect();

    let ids = num().separated_by(whitespace()).allow_trailing().collect();

    let data = ranges
        .then_ignore(newline())
        .then_ignore(newline())
        .then(ids);

    match data.parse(input).into_result() {
        Ok((ranges, ids)) => Some(Input { ranges, ids }),
        Err(errors) => {
            for error in errors {
                println!("Failed to parse input: {}", error);
            }
            None
        }
    }
}

// take vec of possibly-overlapping ranges, and build vector
// containing the start and end of non-overlapping ranges
// obtained by merging overlapping ranges in input
fn consolidate_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<u64> {
    ranges.sort();
    let mut consolidated_ranges = Vec::new();
    let mut current_end = None;
    for (start, end) in ranges {
        if let Some(e) = current_end {
            if start > e {
                consolidated_ranges.push(e);
                consolidated_ranges.push(start);
            }
            if end > e {
                current_end = Some(end);
            }
        } else {
            consolidated_ranges.push(start);
            current_end = Some(end)
        }
    }
    if let Some(end) = current_end {
        consolidated_ranges.push(end);
    }
    consolidated_ranges
}

pub fn part_one(input: &str) -> Option<u64> {
    if let Some(Input { ranges, ids }) = parse(input) {
        let consolidated_ranges = consolidate_ranges(ranges);

        let result = ids
            .iter()
            .filter(|id| {
                let result = consolidated_ranges.binary_search(id);

                if let Err(index) = result
                    && index.is_multiple_of(2)
                {
                    false
                } else {
                    true
                }
            })
            .count();

        Some(result as u64)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Some(Input { ranges, ids: _ }) = parse(input) {
        let consolidated_ranges = consolidate_ranges(ranges);
        let result = consolidated_ranges
            .chunks_exact(2)
            .map(|chunk| chunk[1] - chunk[0] + 1)
            .sum();
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
