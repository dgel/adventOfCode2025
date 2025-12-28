use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

use by_address::ByAddress;
use chumsky::prelude::*;
use chumsky::text::{digits, newline};

use advent_of_code::{KDTree, Point3d, points_dist};

advent_of_code::solution!(8);

fn parse(input: &str) -> Option<Vec<Point3d>> {
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

type Cirquit = Rc<RefCell<Vec<usize>>>;

struct CandidateConnection<I> {
    dist: u64,
    source: usize,
    nearest: Point3d,
    nearest_iter: I,
}

impl<I> Ord for CandidateConnection<I> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl<I> PartialOrd for CandidateConnection<I> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<I> PartialEq for CandidateConnection<I> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<I> Eq for CandidateConnection<I> {}

pub fn part_one_n_connections(input: &str, n_connections: u32) -> Option<u64> {
    if let Some(points) = parse(input) {
        let index_by_point: HashMap<&Point3d, usize> =
            HashMap::from_iter(points.iter().enumerate().map(|(i, point)| (point, i)));

        let kdtree = KDTree::new(&points);

        let mut nearest_heap = BinaryHeap::from_iter(
            points
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    let mut nearest_iter = kdtree.iter_nearest(*point).filter(|&&p| p != *point);
                    if let Some(nearest) = nearest_iter.next() {
                        Some(CandidateConnection {
                            dist: points_dist(point, nearest),
                            source: i,
                            nearest: *nearest,
                            nearest_iter,
                        })
                    } else {
                        None
                    }
                })
                .flatten(),
        );

        let mut num_connections = 0;
        let mut cirquits: Vec<Cirquit> = points
            .iter()
            .enumerate()
            .map(|(i, _)| Rc::new(RefCell::new(vec![i])))
            .collect();

        let mut seen_pairs = HashSet::new();
        while let Some(CandidateConnection {
            dist: _,
            source: i,
            nearest,
            mut nearest_iter,
        }) = nearest_heap.pop()
        {
            let j = index_by_point[&nearest];
            let inserted = seen_pairs.insert(if i < j { (i, j) } else { (j, i) });
            if inserted {
                if !Rc::ptr_eq(&cirquits[i], &cirquits[j]) {
                    let (larger, smaller) =
                        if cirquits[i].borrow().len() > cirquits[j].borrow().len() {
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

            if let Some(nearest) = nearest_iter.next() {
                nearest_heap.push(CandidateConnection {
                    dist: points_dist(&points[i], nearest),
                    source: i,
                    nearest: *nearest,
                    nearest_iter,
                });
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
        let index_by_point: HashMap<&Point3d, usize> =
            HashMap::from_iter(points.iter().enumerate().map(|(i, point)| (point, i)));

        let kdtree = KDTree::new(&points);

        let mut nearest_heap = BinaryHeap::from_iter(
            points
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    let mut nearest_iter = kdtree.iter_nearest(*point).filter(|&&p| p != *point);
                    if let Some(nearest) = nearest_iter.next() {
                        Some(CandidateConnection {
                            dist: points_dist(point, nearest),
                            source: i,
                            nearest: *nearest,
                            nearest_iter,
                        })
                    } else {
                        None
                    }
                })
                .flatten(),
        );

        let mut num_connections = 0;
        let mut cirquits: Vec<Cirquit> = points
            .iter()
            .enumerate()
            .map(|(i, _)| Rc::new(RefCell::new(vec![i])))
            .collect();

        let mut seen_pairs = HashSet::new();
        while let Some(CandidateConnection {
            dist: _,
            source: i,
            nearest,
            mut nearest_iter,
        }) = nearest_heap.pop()
        {
            let j = index_by_point[&nearest];
            let inserted = seen_pairs.insert(if i < j { (i, j) } else { (j, i) });
            if inserted {
                if !Rc::ptr_eq(&cirquits[i], &cirquits[j]) {
                    let (larger, smaller) =
                        if cirquits[i].borrow().len() > cirquits[j].borrow().len() {
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

            if let Some(nearest) = nearest_iter.next() {
                nearest_heap.push(CandidateConnection {
                    dist: points_dist(&points[i], nearest),
                    source: i,
                    nearest: *nearest,
                    nearest_iter,
                });
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
