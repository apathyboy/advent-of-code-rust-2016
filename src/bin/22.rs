use glam::IVec2;
use pathfinding::prelude::bfs;
use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(22);

#[derive(Debug, Clone)]
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

struct StorageCluster {
    nodes: Vec<Node>,
    width: i32,
    height: i32,
}

impl StorageCluster {
    fn new(nodes: Vec<Node>) -> Self {
        let width = nodes.iter().map(|n| n.pos.x).max().unwrap() + 1;
        let height = nodes.iter().map(|n| n.pos.y).max().unwrap() + 1;
        StorageCluster {
            nodes,
            width,
            height,
        }
    }

    fn get_offset(&self, in_x: i32, in_y: i32) -> usize {
        ((self.height - 1 - in_y) * in_x + in_x + (in_x * in_y + in_y)) as usize
    }

    fn in_bounds(&self, test_pos: &IVec2) -> bool {
        test_pos.x >= 0 && test_pos.y >= 0 && test_pos.x < self.width && test_pos.y < self.height
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

fn successors(pos: &IVec2, cluster: &StorageCluster, max_size: u32) -> Vec<IVec2> {
    //let mut successors = Vec::new();
    let directions = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    directions
        .iter()
        .filter_map(|dir| {
            let test_pos = *pos + *dir;

            if !cluster.in_bounds(&test_pos) {
                return None;
            }

            let test_offset = cluster.get_offset(test_pos.x, test_pos.y);
            let test_node = &cluster.nodes[test_offset];

            if test_node.used <= max_size {
                Some(test_node.pos)
            } else {
                None
            }
        })
        .collect()
}

fn fewest_steps(start: &Node, goal: &Node, cluster: &StorageCluster) -> usize {
    let result = bfs(
        &start.pos,
        |p| successors(p, cluster, start.size),
        |p| *p == goal.pos,
    );

    result.expect("no path found").len() - 1
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

pub fn part_two(input: &str) -> Option<usize> {
    let cluster = StorageCluster::new(input.lines().skip(2).filter_map(parse_line).collect());

    let target_node = cluster
        .nodes
        .iter()
        .filter(|&n| n.pos.y == 0)
        .max_by(|&a, &b| a.pos.x.cmp(&b.pos.x))
        .unwrap();

    let empty_node = cluster.nodes.iter().find(|&n| n.used == 0).unwrap();

    let steps = fewest_steps(empty_node, target_node, &cluster);

    Some(steps + ((cluster.width as usize - 2) * 5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
