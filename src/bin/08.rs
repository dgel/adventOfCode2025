use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use by_address::ByAddress;
use chumsky::prelude::*;
use chumsky::text::{digits, newline};

advent_of_code::solution!(8);

type Point = [u64; 3];

fn parse(input: &str) -> Option<Vec<Point>> {
    let num = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };
    let point = num()
        .then(just(',').ignore_then(num()))
        .then(just(',').ignore_then(num()))
        .map(|((x, y), z)| [x, y, z]);
    let points = point.separated_by(newline()).allow_trailing().collect();

    match points.parse(input).into_result() {
        Ok(points) => Some(points),
        Err(errors) => {
            for error in errors {
                println!("Failed to parse input: {}", error);
            }
            None
        }
    }
}

fn distance(points: &[Point], p1: usize, p2: usize) -> u64 {
    let p1 = points[p1];
    let p2 = points[p2];
    (0..3)
        .map(|d| {
            let diff = p1[d].abs_diff(p2[d]);
            diff * diff
        })
        .sum()
}

type Cirquit = Rc<RefCell<Vec<usize>>>;

pub fn part_one_n_connections(input: &str, n_connections: u32) -> Option<u64> {
    if let Some(points) = parse(input) {
        let mut distances = Vec::new();

        // TODO: perhaps replace with kd-tree?
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                distances.push((distance(&points, i, j), i, j))
            }
        }
        distances.sort();
        let mut num_connections = 0;
        let mut cirquits: Vec<Cirquit> = points
            .iter()
            .enumerate()
            .map(|(i, _)| Rc::new(RefCell::new(vec![i])))
            .collect(); //vec![None; points.len()];

        for (_, i, j) in distances {
            if Rc::ptr_eq(&cirquits[i], &cirquits[j]) {
            } else {
                let (larger, smaller) = if cirquits[i].borrow().len() > cirquits[j].borrow().len() {
                    (i, j)
                } else {
                    (j, i)
                };

                let larger = cirquits[larger].clone();
                for &junction in cirquits[smaller].clone().borrow().iter() {
                    larger.borrow_mut().push(junction);
                    cirquits[junction] = larger.clone();
                }
            }
            num_connections += 1;

            if num_connections >= n_connections {
                break;
            }
        }

        let mut unique_cirquits: Vec<Cirquit> = cirquits
            .into_iter()
            .map(ByAddress)
            .collect::<HashSet<ByAddress<Cirquit>>>()
            .into_iter()
            .map(|f| f.0)
            .collect();

        unique_cirquits.sort_by_key(|s| std::cmp::Reverse(s.borrow().len()));
        Some(
            unique_cirquits
                .iter()
                .take(3)
                .map(|s| s.borrow().len() as u64)
                .product::<u64>(),
        )
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_n_connections(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Some(points) = parse(input) {
        let mut distances = Vec::new();

        // TODO: perhaps replace with kd-tree?
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                distances.push((distance(&points, i, j), i, j))
            }
        }
        distances.sort();
        let mut num_connections = 0;
        let mut cirquits: Vec<Cirquit> = points
            .iter()
            .enumerate()
            .map(|(i, _)| Rc::new(RefCell::new(vec![i])))
            .collect(); //vec![None; points.len()];

        for (_, i, j) in distances {
            if Rc::ptr_eq(&cirquits[i], &cirquits[j]) {
            } else {
                let (larger, smaller) = if cirquits[i].borrow().len() > cirquits[j].borrow().len() {
                    (i, j)
                } else {
                    (j, i)
                };

                let larger = cirquits[larger].clone();
                for &junction in cirquits[smaller].clone().borrow().iter() {
                    larger.borrow_mut().push(junction);
                    cirquits[junction] = larger.clone();
                }
                num_connections += 1;
            }

            if num_connections == points.len() - 1 {
                let [x1, _, _] = points[i];
                let [x2, _, _] = points[j];
                return Some(x1 * x2);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_n_connections(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
