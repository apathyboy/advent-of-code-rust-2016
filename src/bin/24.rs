use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Clone)]
enum NodeType {
    Wall,
    Open,
}

#[derive(Debug)]
struct PointOfInterest {
    id: u8,
    position: IVec2,
}

impl PointOfInterest {
    fn new(id: u8, position: IVec2) -> Self {
        Self { id, position }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge {
    a: u8,
    b: u8,
    weight: u32,
}

impl Edge {
    fn new(a: u8, b: u8, weight: u32) -> Self {
        Self { a, b, weight }
    }
}

#[derive(Debug)]
struct Map {
    nodes: HashMap<IVec2, NodeType>,
    poi: Vec<PointOfInterest>,
    width: i32,
    height: i32,
}

impl Map {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            poi: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn successors(&self, pos: &IVec2) -> Vec<IVec2> {
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
                let node_type = self.nodes.get(&test_pos);

                node_type.and_then(|t| match t {
                    NodeType::Open => Some(test_pos),
                    NodeType::Wall => None,
                })
            })
            .collect()
    }
}

fn parse_map(input: &str) -> Option<Map> {
    let mut map = Map::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, row) in input.lines().enumerate() {
        if y > max_y {
            max_y = y;
        }

        for (x, cell) in row.chars().enumerate() {
            if x > max_x {
                max_x = x;
            }

            let pos = IVec2::new(x as i32, y as i32);
            let node_type = match cell {
                '#' => NodeType::Wall,
                _ => NodeType::Open,
            };

            map.nodes.insert(pos, node_type.clone());

            if node_type == NodeType::Open && cell != '.' {
                // is poi
                let poi = PointOfInterest::new(cell.to_digit(10).unwrap() as u8, pos);

                map.poi.push(poi);
            }
        }
    }

    map.poi.sort_by(|a, b| a.id.cmp(&b.id));

    map.width = max_x as i32 + 1;
    map.height = max_y as i32 + 1;

    Some(map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input)?;

    let start_node = &map.poi[0];

    let mut edges = Vec::new();

    for goal in map.poi.iter().skip(1) {
        let result = bfs(
            &start_node.position,
            |p| map.successors(p),
            |p| *p == goal.position,
        )
        .unwrap();

        edges.push(Edge::new(start_node.id, goal.id, result.len() as u32 - 1));
    }

    for a in map.poi.iter().skip(1).permutations(2) {
        let result = bfs(
            &a[0].position,
            |p| map.successors(p),
            |p| *p == a[1].position,
        )
        .unwrap();

        edges.push(Edge::new(a[0].id, a[1].id, result.len() as u32 - 1));
    }

    let must_visit_start = 1;
    let must_visit_end = map.poi.last().unwrap().id;
    let must_visit = must_visit_start..=must_visit_end;

    must_visit
        .permutations(must_visit_end as usize)
        .map(|p| {
            let mut prev = 0;
            let mut len = 0;
            for n in p {
                len += edges
                    .iter()
                    .find(|e| e.a == prev && e.b == n)
                    .unwrap()
                    .weight;
                prev = n;
            }

            len
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input)?;

    let mut edges = Vec::new();

    for a in map.poi.iter().permutations(2) {
        let result = bfs(
            &a[0].position,
            |p| map.successors(p),
            |p| *p == a[1].position,
        )
        .unwrap();

        edges.push(Edge::new(a[0].id, a[1].id, result.len() as u32 - 1));
    }

    let must_visit_start = 1;
    let must_visit_end = map.poi.last().unwrap().id;
    let must_visit = must_visit_start..=must_visit_end;

    must_visit
        .permutations(must_visit_end as usize)
        .map(|p| {
            let mut prev = 0;
            let mut len = 0;
            for n in p {
                len += edges
                    .iter()
                    .find(|e| e.a == prev && e.b == n)
                    .unwrap()
                    .weight;
                prev = n;
            }

            len += edges
                .iter()
                .find(|e| e.a == prev && e.b == 0)
                .unwrap()
                .weight;

            len
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
