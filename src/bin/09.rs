use chumsky::prelude::*;
use chumsky::text::{digits, newline};
advent_of_code::solution!(9);

use advent_of_code::{Area, AreaKDTree};

type Point = [u64; 2];

fn parse(input: &str) -> Option<Vec<Point>> {
    let num = || {
        digits::<&str, extra::Err<Rich<char>>>(10)
            .to_slice()
            .from_str()
            .unwrapped()
    };
    let point = num()
        .then(just(',').ignore_then(num()))
        .map(|(x, y)| [x, y]);
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

pub fn part_one(input: &str) -> Option<u64> {
    if let Some(points) = parse(input) {
        let mut max_size = 0;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let [x1, y1] = points[i];
                let [x2, y2] = points[j];

                let size = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
                if size > max_size {
                    max_size = size;
                }
            }
        }
        Some(max_size)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Some(points) = parse(input) {
        if points.len() < 2 {
            return None;
        }
        let mut segments: Vec<Area> = points
            .windows(2)
            .map(|w| Area::from_points(w[0], w[1]))
            .collect();
        segments.push(Area::from_points(
            *points.last().unwrap(),
            *points.first().unwrap(),
        ));

        let kdtree = AreaKDTree::new(&segments);
        // kdtree.print();

        let mut max_size = 0;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let area = Area::from_points(points[i], points[j]);

                let size = area.size();
                if size > max_size && !kdtree.any_overlapping(&area) {
                    max_size = size;
                }
            }
        }
        Some(max_size)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
