use std::collections::{HashMap, VecDeque};

use chumsky::prelude::*;
use chumsky::text::newline;

advent_of_code::solution!(11);

fn parse(input: &str) -> Option<Vec<(String, Vec<String>)>> {
    let name = any::<&str, extra::Err<Rich<char>>>()
        .filter(|ch| ch.is_alphabetic())
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|slice: &str| slice.to_owned())
        .padded_by(just(' ').repeated());
    let entry = name
        .then_ignore(just(':'))
        .then(name.repeated().at_least(1).collect());

    let entries = entry.separated_by(newline()).allow_trailing().collect();

    match entries.parse(input).into_result() {
        Ok(entries) => Some(entries),
        Err(errors) => {
            for error in errors {
                println!("Failed to parse input: {}", error);
            }
            None
        }
    }
}

struct GraphEntry<'a> {
    name: &'a str,
    connections: Vec<usize>,
}

fn get_index<'a>(
    name: &'a str,
    index_map: &mut HashMap<&'a str, usize>,
    graph: &mut Vec<GraphEntry<'a>>,
) -> usize {
    let index = *index_map.entry(name).or_insert(graph.len());
    if index == graph.len() {
        graph.push(GraphEntry {
            name,
            connections: Vec::new(),
        });
    }
    index
}

fn build_graph<'a>(
    entries: &'a [(String, Vec<String>)],
) -> (HashMap<&'a str, usize>, Vec<GraphEntry<'a>>, Vec<usize>) {
    let mut index_map = HashMap::with_capacity(entries.len());
    let mut graph = Vec::with_capacity(entries.len());
    let mut incoming_connections = Vec::with_capacity(entries.len());
    for (source, targets) in entries {
        let index = get_index(source, &mut index_map, &mut graph);
        for target in targets {
            let target_index = get_index(target, &mut index_map, &mut graph);
            if target_index >= incoming_connections.len() {
                incoming_connections.resize(target_index + 1, 0);
            }

            graph[index].connections.push(target_index);
            incoming_connections[target_index] += 1;
        }
    }

    let mut queue = VecDeque::new();
    for (idx, _) in graph.iter().enumerate() {
        if incoming_connections[idx] == 0 {
            queue.push_back(idx);
        }
    }

    let mut ordered = Vec::with_capacity(incoming_connections.len());
    while let Some(idx) = queue.pop_front() {
        for &target in &graph[idx].connections {
            incoming_connections[target] -= 1;
            if incoming_connections[target] == 0 {
                queue.push_back(target);
            }
        }
        ordered.push(idx);
    }
    (index_map, graph, ordered)
}

fn number_of_paths(
    graph: &[GraphEntry],
    ordered_entries: &[usize],
    start_index: usize,
    end_index: usize,
) -> u64 {
    let mut ways_to_reach = vec![0; graph.len()];
    ways_to_reach[start_index] = 1;

    for &idx in ordered_entries {
        for &target in &graph[idx].connections {
            ways_to_reach[target] += ways_to_reach[idx];
        }
    }
    ways_to_reach[end_index]
}

pub fn part_one(input: &str) -> Option<u64> {
    if let Some(entries) = parse(input) {
        let (index_map, graph, ordered) = build_graph(&entries);
        let start_index = *index_map.get("you").unwrap();
        let end_index = *index_map.get("out").unwrap();
        Some(number_of_paths(&graph, &ordered, start_index, end_index))
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Some(entries) = parse(input) {
        let (index_map, graph, ordered) = build_graph(&entries);
        let start_index = *index_map.get("svr").unwrap();
        let fft_index = *index_map.get("fft").unwrap();
        let dac_index = *index_map.get("dac").unwrap();
        let end_index = *index_map.get("out").unwrap();

        // two possibilities: you -> fft -> dac -> out
        //                    you -> dac -> fft -> out

        let path1 = number_of_paths(&graph, &ordered, start_index, fft_index)
            * number_of_paths(&graph, &ordered, fft_index, dac_index)
            * number_of_paths(&graph, &ordered, dac_index, end_index);
        let path2 = number_of_paths(&graph, &ordered, start_index, dac_index)
            * number_of_paths(&graph, &ordered, dac_index, fft_index)
            * number_of_paths(&graph, &ordered, fft_index, end_index);

        Some(path1 + path2)
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
