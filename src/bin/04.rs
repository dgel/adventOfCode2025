use std::mem::swap;

use chumsky::prelude::*;
use chumsky::text::whitespace;

advent_of_code::solution!(4);

#[derive(Eq, PartialEq, Copy, Clone)]
enum Content {
    Empty,
    Roll,
}

fn parse(input: &str) -> Option<Vec<Vec<Content>>> {
    let empty = just::<_, _, extra::Err<Rich<char>>>('.').map(|_| Content::Empty);
    let roll = just('@').map(|_| Content::Roll);
    let space = choice((empty, roll));
    let row = space.repeated().at_least(1).collect();
    let rows = row.separated_by(whitespace()).allow_trailing().collect();
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

fn is_accessible(rows: &[Vec<Content>], position: (isize, isize)) -> bool {
    let mut n_adjacent_occupied = 0;
    for (y_offset, x_offset) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (y, x) = (position.0 + y_offset, position.1 + x_offset);
        if y >= 0 && x >= 0 {
            let (y, x) = (y as usize, x as usize);
            if y < rows.len() {
                let row = &rows[y];
                if x < row.len() && row[x] == Content::Roll {
                    n_adjacent_occupied += 1;
                }
            }
        }
    }
    n_adjacent_occupied < 4
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse(input);
    data.map(|rows| {
        rows.iter()
            .enumerate()
            .map(|(y, row)| {
                (0..row.len())
                    .map(|x| {
                        if row[x] == Content::Roll && is_accessible(&rows, (y as isize, x as isize))
                        {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u64>()
            })
            .sum()
    })
}

fn remove_rolls(rows: &[Vec<Content>], result: &mut Vec<Vec<Content>>) -> u64 {
    result.resize(rows.len(), Vec::new());
    let mut removed_rolls = 0;

    rows.iter().enumerate().for_each(|(y, row)| {
        let output_row = &mut result[y];
        output_row.resize(row.len(), Content::Empty);
        (0..row.len()).for_each(|x| {
            let new_value = if row[x] == Content::Roll {
                if is_accessible(rows, (y as isize, x as isize)) {
                    removed_rolls += 1;
                    Content::Empty
                } else {
                    Content::Roll
                }
            } else {
                Content::Empty
            };
            output_row[x] = new_value;
        })
    });
    removed_rolls
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse(input);

    if let Some(mut rows) = data {
        let mut new_rows = Vec::new();
        let mut total_removed_rolls = 0;
        loop {
            let removed_rolls = remove_rolls(&rows, &mut new_rows);
            if removed_rolls == 0 {
                return Some(total_removed_rolls);
            }
            total_removed_rolls += removed_rolls;
            swap(&mut rows, &mut new_rows);
        }
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
