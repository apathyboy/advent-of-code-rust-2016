use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(22);

struct Node {
    name: String,
    size: u32,
    used: u32,
    avail: u32,
    usage: u32,
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r"\/dev\/grid\/(node-x[0-9]+-y[0-9]+)\s+([0-9]+)T\s+([0-9]+)T\s+([0-9]+)T\s+([0-9]+)%"
    )
    .unwrap();
}

fn parse_line(line: &str) -> Option<Node> {
    let (name, size, used, avail, usage) = LINE_REGEX
        .captures(line)
        .map(|cap| {
            (
                cap[1].to_string(),
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[5].parse::<u32>().unwrap(),
            )
        })
        .unwrap();

    Some(Node {
        name,
        size,
        used,
        avail,
        usage,
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let nodes: Vec<_> = input.lines().skip(2).filter_map(parse_line).collect();

    let mut viable_pairs = HashSet::new();

    for node1 in nodes.iter() {
        if node1.used == 0 {
            continue;
        }

        for node2 in nodes.iter() {
            if node1.name == node2.name || node1.used > node2.avail || node1.used > node2.size {
                continue;
            }

            let min_node = std::cmp::min(&node1.name, &node2.name);
            let max_node = std::cmp::max(&node1.name, &node2.name);

            viable_pairs.insert((min_node, max_node));
        }
    }

    Some(viable_pairs.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
