use std::mem::swap;

advent_of_code::solution!(7);

#[derive(Debug)]
enum Location {
    Start,
    Split,
    Empty,
}

fn parse(input: &str) -> Vec<Vec<Location>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'S' => Location::Start,
                    b'^' => Location::Split,
                    _ => Location::Empty,
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse(input);

    let width = map.first().map(|m| m.len()).unwrap_or(0);
    let mut beams = vec![false; width];
    let mut new_beams = vec![false; width];

    let mut split_counts = 0;
    for line in map {
        for (i, location) in line.iter().enumerate() {
            let has_beam = beams[i];
            match location {
                Location::Start => {
                    if i < new_beams.len() {
                        new_beams[i] = true;
                    }
                }
                Location::Split => {
                    if has_beam {
                        split_counts += 1;
                        if i > 0 {
                            new_beams[i - 1] = true;
                        }
                        if i + 1 < new_beams.len() {
                            new_beams[i + 1] = true;
                        }
                    }
                }
                _ => {
                    if has_beam && i < new_beams.len() {
                        new_beams[i] = true
                    }
                }
            }
        }
        swap(&mut beams, &mut new_beams);
        new_beams.fill(false);
    }
    Some(split_counts)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse(input);

    let width = map.first().map(|m| m.len()).unwrap_or(0);
    let mut timelines = vec![0; width];
    let mut new_timelines = vec![0; width];

    for line in map {
        for (i, location) in line.iter().enumerate() {
            let n_timelines = timelines[i];
            match location {
                Location::Start => {
                    if i < new_timelines.len() {
                        new_timelines[i] += 1;
                    }
                }
                Location::Split => {
                    if n_timelines > 0 {
                        if i > 0 {
                            new_timelines[i - 1] += n_timelines;
                        }
                        if i + 1 < new_timelines.len() {
                            new_timelines[i + 1] += n_timelines;
                        }
                    }
                }
                _ => {
                    new_timelines[i] += n_timelines;
                }
            }
        }
        swap(&mut timelines, &mut new_timelines);
        new_timelines.fill(0);
    }
    Some(timelines.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
