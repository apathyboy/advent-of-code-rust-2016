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

fn get_offset(x: i32, y: i32) -> usize {
    ((27 - y) * x + x + (x * y + y)) as usize
}

fn successors(pos: &IVec2, nodes: &[Node], max_size: u32) -> Vec<IVec2> {
    let mut successors = Vec::new();

    // up
    if pos.y > 0 {
        let up_pos = *pos + IVec2::new(0, -1);
        let up_offset = get_offset(up_pos.x, up_pos.y);
        let up_node = &nodes[up_offset];

        if up_node.used <= max_size {
            successors.push(up_node.pos);
        }
    }

    // down
    if pos.y < 27 {
        let down_pos = *pos + IVec2::new(0, 1);
        let down_offset = get_offset(down_pos.x, down_pos.y);
        let down_node = &nodes[down_offset];

        if down_node.used <= max_size {
            successors.push(down_node.pos);
        }
    }

    // left
    if pos.x > 0 {
        let left_pos = *pos + IVec2::new(-1, 0);
        let left_offset = get_offset(left_pos.x, left_pos.y);
        let left_node = &nodes[left_offset];

        if left_node.used <= max_size {
            successors.push(left_node.pos);
        }
    }

    // right
    if pos.x < 36 {
        let right_pos = *pos + IVec2::new(1, 0);
        let right_offset = get_offset(right_pos.x, right_pos.y);
        let right_node = &nodes[right_offset];

        if right_node.used <= max_size {
            successors.push(right_node.pos);
        }
    }

    successors
}

fn fewest_steps(start: &Node, goal: &Node, nodes: &[Node]) -> usize {
    let result = bfs(
        &start.pos,
        |p| successors(p, nodes, start.size),
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

    let steps = fewest_steps(empty_node, target_node, &cluster.nodes);

    Some(steps + (35 * 5))
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
