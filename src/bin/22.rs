use glam::IVec2;
use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(22);

#[derive(Debug)]
struct Node {
    pos: IVec2,
    size: u32,
    used: u32,
}

impl Node {
    fn new(pos: IVec2, size: u32, used: u32) -> Self {
        Node { pos, size, used }
    }

    fn name(&self) -> String {
        format!("node-x{}-y{}", self.pos.x, self.pos.y)
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r"\/dev\/grid\/node-x([0-9]+)-y([0-9]+)\s+([0-9]+)T\s+([0-9]+)T\s+([0-9]+)T\s+([0-9]+)%"
    )
    .unwrap();
}

fn parse_line(line: &str) -> Option<Node> {
    let (x, y, size, used) = LINE_REGEX
        .captures(line)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
            )
        })
        .unwrap();

    Some(Node::new(IVec2::new(x, y), size, used))
}

pub fn part_one(input: &str) -> Option<usize> {
    let nodes: Vec<_> = input.lines().skip(2).filter_map(parse_line).collect();

    let mut viable_pairs = HashSet::new();

    for node1 in nodes.iter() {
        if node1.used == 0 {
            continue;
        }

        for node2 in nodes.iter() {
            if node1.pos == node2.pos || node1.used > (node2.size - node2.used) {
                continue;
            }

            let min_node = std::cmp::min(node1.name(), node2.name());
            let max_node = std::cmp::max(node1.name(), node2.name());

            viable_pairs.insert((min_node, max_node));
        }
    }

    Some(viable_pairs.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let nodes: Vec<_> = input.lines().skip(2).filter_map(parse_line).collect();

    let target_data = nodes
        .iter()
        .filter(|&n| n.pos.y == 0)
        .max_by(|&a, &b| a.pos.x.cmp(&b.pos.x))
        .unwrap();

    let empty_node = nodes.iter().find(|&n| n.used == 0).unwrap();

    println!("Target data: {:?}", target_data);
    println!("Empty node: {:?}", empty_node);

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
