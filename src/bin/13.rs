use std::collections::{HashSet, VecDeque};

use glam::UVec2;
use pathfinding::prelude::bfs;

advent_of_code::solution!(13);

fn successors(pos: &UVec2, fav_num: u32) -> Vec<UVec2> {
    let mut successors = Vec::new();

    // up
    if pos.y != 0 && !is_wall(pos.x, pos.y.checked_sub(1).unwrap(), fav_num) {
        successors.push(*pos - UVec2::new(0, 1));
    }

    // down
    if !is_wall(pos.x, pos.y.checked_add(1).unwrap(), fav_num) {
        successors.push(*pos + UVec2::new(0, 1));
    }

    // left
    if pos.x != 0 && !is_wall(pos.x.checked_sub(1).unwrap(), pos.y, fav_num) {
        successors.push(*pos - UVec2::new(1, 0));
    }

    if !is_wall(pos.x.checked_add(1).unwrap(), pos.y, fav_num) {
        successors.push(*pos + UVec2::new(1, 0));
    }

    successors
}

fn is_wall(x: u32, y: u32, fav_num: u32) -> bool {
    let val = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + fav_num;
    let bin = format!("{:b}", val);

    bin.chars().filter(|c| *c == '1').count() % 2 != 0
}

fn fewest_steps(start: UVec2, goal: UVec2, fav_num: u32) -> usize {
    let result = bfs(&start, |p| successors(p, fav_num), |p| *p == goal);

    result.expect("no path found").len() - 1
}

fn reachable_locations(start: UVec2, steps: usize, fav_num: u32) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((pos, step)) = queue.pop_front() {
        if step >= steps {
            break;
        }

        for new_pos in successors(&pos, fav_num) {
            if !visited.contains(&new_pos) {
                queue.push_back((new_pos, step + 1));
                visited.insert(new_pos);
            }
        }
    }

    visited.len()
}

fn parse_fav_number(input: &str) -> Option<u32> {
    input.trim().parse::<u32>().ok()
}

pub fn part_one(input: &str) -> Option<usize> {
    let fav_num = parse_fav_number(input).unwrap();

    let start = UVec2::new(1, 1);
    let goal = UVec2::new(31, 39);

    Some(fewest_steps(start, goal, fav_num))
}

pub fn part_two(input: &str) -> Option<usize> {
    let fav_num = parse_fav_number(input).unwrap();

    let start = UVec2::new(1, 1);

    Some(reachable_locations(start, 50, fav_num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fav_number() {
        let result = parse_fav_number(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_fewest_steps() {
        let fav_num = 10;
        let start = UVec2::new(1, 1);
        let goal = UVec2::new(7, 4);
        let result = fewest_steps(start, goal, fav_num);

        assert_eq!(result, 11);
    }
}
