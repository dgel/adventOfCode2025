use chumsky::prelude::*;
use chumsky::text::{digits, whitespace};

advent_of_code::solution!(1);

fn parse(input: &str) -> Option<Vec<i64>> {
    let left = just::<_, _, extra::Err<Rich<char>>>('L')
        .ignore_then(digits(10).to_slice())
        .from_str::<i64>()
        .unwrapped()
        .map(|i| -i);
    let right = just('R')
        .ignore_then(digits(10).to_slice())
        .from_str::<i64>()
        .unwrapped();
    let either = choice((left, right));
    let parser = either
        .separated_by(whitespace())
        .allow_trailing()
        .collect::<Vec<i64>>();
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

pub fn part_one(input: &str) -> Option<u64> {
    let turns = parse(input);
    turns.map(|v| {
        v.iter()
            .scan(50, |st, &v| {
                let summed = *st + v;
                *st = summed.rem_euclid(100);
                Some(*st)
            })
            .filter(|&n| n == 0)
            .count() as u64
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let turns = parse(input);
    turns.map(|v| {
        v.iter()
            .scan(50, |st, &v| {
                let pre = *st;
                let summed = pre + v;
                let mut num_turns = (summed / 100).abs();
                *st = summed.rem_euclid(100);
                if summed <= 0 && pre != 0 {
                    num_turns += 1;
                }

                Some(num_turns)
            })
            .sum::<i64>() as u64
    })
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
        assert_eq!(result, Some(6));
    }
}
