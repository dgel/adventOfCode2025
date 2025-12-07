use chumsky::prelude::*;
use chumsky::text::{digits, newline, whitespace};

advent_of_code::solution!(6);

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
}

struct Input {
    num_lines: Vec<Vec<u64>>,
    instructions: Vec<Operation>,
}

fn parse(input: &str) -> Option<Input> {
    let num = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };
    let nums = just(' ').repeated().ignore_then(
        num()
            .separated_by(just(' ').repeated().at_least(1))
            .at_least(1)
            .allow_trailing()
            .collect(),
    );
    let num_lines = nums.separated_by(newline()).allow_trailing().collect();

    let instruction = choice((
        just('*').map(|_| Operation::Multiply),
        just('+').map(|_| Operation::Add),
    ));
    let instructions = just(' ')
        .repeated()
        .ignore_then(
            instruction
                .separated_by(just(' ').repeated().at_least(1))
                .collect(),
        )
        .then_ignore(whitespace());

    let data = num_lines.then(instructions);

    match data.parse(input).into_result() {
        Ok((num_lines, instructions)) => Some(Input {
            num_lines,
            instructions,
        }),
        Err(errors) => {
            for error in errors {
                println!("Failed to parse input: {}", error);
            }
            None
        }
    }
}

fn transpose(input: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    if input.is_empty() {
        return Vec::new();
    }
    let len = input[0].len();
    for (i, line) in input.iter().enumerate() {
        if line.len() != len {
            println!(
                "Input line {} has unexpected length {}, expected {}",
                i,
                line.len(),
                len
            );
            return Vec::new();
        }
    }
    (0..len)
        .map(|index| input.iter().map(|line| line[index]).collect())
        .collect()
}

fn calculate(values: &[Vec<u64>], instructions: &[Operation]) -> u64 {
    values
        .iter()
        .zip(instructions)
        .map(|(nums, operation)| match operation {
            Operation::Multiply => nums.iter().product::<u64>(),
            Operation::Add => nums.iter().sum(),
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    if let Some(Input {
        num_lines,
        instructions,
    }) = parse(input)
    {
        let values = transpose(num_lines);
        if values.len() != instructions.len() {
            println!(
                "Mismatch between number of values columns ({}) and number of instructions ({})",
                values.len(),
                instructions.len()
            );
            return None;
        }
        Some(calculate(&values, &instructions))
    } else {
        None
    }
}

fn parse_columnar(input: &str) -> Option<Input> {
    let lines: Vec<&str> = input.lines().collect();

    let num_lines = &lines.as_slice()[0..lines.len() - 1];
    let instructions = lines.last();
    if let (num_lines, Some(instructions)) = (num_lines, instructions) {
        let num_lines: Vec<Vec<u8>> = num_lines
            .iter()
            .map(|line| line.bytes().collect())
            .collect();
        if num_lines.is_empty() {
            return None;
        }
        let len = num_lines[0].len();
        for (i, num_line) in num_lines.iter().enumerate() {
            if num_line.len() != len {
                println!(
                    "Input line {} has unexpected length {}, expected {}",
                    i,
                    num_line.len(),
                    len
                );
                return None;
            }
        }

        let mut values = Vec::new();
        let mut cur_nums = Vec::new();
        let mut cur_num: Option<u64> = None;
        for i in 0..len {
            for line in num_lines.iter() {
                let byte = line[i];
                if byte.is_ascii_digit() {
                    let byte_val = (byte - b'0') as u64;
                    cur_num = Some(cur_num.map_or(byte_val, |num| num * 10 + byte_val));
                }
            }
            if let Some(num) = cur_num {
                cur_nums.push(num);
                cur_num = None;
            } else {
                values.push(cur_nums.clone());
                cur_nums.clear();
            }
        }
        values.push(cur_nums);

        let instrs: Vec<Operation> = instructions
            .split_whitespace()
            .map(|instr| {
                if instr == "*" {
                    Operation::Multiply
                } else {
                    Operation::Add
                }
            })
            .collect();
        if instrs.len() != values.len() {
            println!(
                "Mismatch between number of values columns ({}) and number of instructions ({})",
                values.len(),
                instrs.len()
            );
            return None;
        }
        Some(Input {
            num_lines: values,
            instructions: instrs,
        })
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Some(Input {
        num_lines,
        instructions,
    }) = parse_columnar(input)
    {
        Some(calculate(&num_lines, &instructions))
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
