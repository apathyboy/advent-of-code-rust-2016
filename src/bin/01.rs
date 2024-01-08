use glam::IVec2;
use itertools::Itertools;
use num_traits::abs;
use num_traits::signum;

advent_of_code::solution!(1);

enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct LineSegment {
    start: IVec2,
    end: IVec2,
}

impl LineSegment {
    fn new(a: IVec2, b: IVec2) -> Self {
        if b.x < a.x || b.y < a.y {
            Self { start: b, end: a }
        } else {
            Self { start: a, end: b }
        }
    }
}

fn turn(facing: &IVec2, turn_dir: TurnDirection) -> IVec2 {
    match facing {
        IVec2 { x, y: 0 } => match turn_dir {
            TurnDirection::Left => IVec2::new(0, signum(*x)),
            TurnDirection::Right => IVec2::new(0, -signum(*x)),
        },
        IVec2 { x: 0, y } => match turn_dir {
            TurnDirection::Left => IVec2::new(-signum(*y), 0),
            TurnDirection::Right => IVec2::new(signum(*y), 0),
        },
        _ => panic!("Invalid direction"),
    }
}

fn intersects_at(segment1: &LineSegment, segment2: &LineSegment) -> Option<IVec2> {
    if (segment1.start.x == segment1.end.x && segment2.start.x == segment2.end.x)
        || (segment1.start.y == segment1.end.y && segment2.start.y == segment2.end.y)
    {
        return None;
    }

    if segment1.start.x == segment1.end.x
        && segment2.start.x < segment1.start.x
        && segment2.end.x > segment1.start.x
        && segment1.start.y < segment2.start.y
        && segment1.end.y > segment2.end.y
    {
        return Some(IVec2::new(segment1.start.x, segment2.start.y));
    }

    if segment1.start.y == segment1.end.y
        && segment2.start.y < segment1.start.y
        && segment2.end.y > segment1.start.y
        && segment1.start.x < segment2.start.x
        && segment1.end.x > segment2.end.x
    {
        return Some(IVec2::new(segment2.start.x, segment1.start.y));
    }

    None
}

fn find_intersection(visited: &[IVec2], segment1: &LineSegment) -> Option<IVec2> {
    visited.iter().tuple_windows().find_map(|(start, end)| {
        let segment2 = LineSegment::new(*start, *end);
        intersects_at(segment1, &segment2)
    })
}

fn parse_instruction(instruction: &str) -> Option<(TurnDirection, i32)> {
    let turn_direction = match &instruction[0..1] {
        "L" => TurnDirection::Left,
        "R" => TurnDirection::Right,
        _ => panic!("Invalid instruction"),
    };

    let distance = instruction[1..].parse::<i32>().unwrap();

    Some((turn_direction, distance))
}

pub fn part_one(input: &str) -> Option<i32> {
    let (final_pos, _) = input.trim().split(", ").filter_map(parse_instruction).fold(
        (IVec2::new(0, 0), IVec2::new(0, 1)),
        |(mut pos, mut facing), (turn_direction, distance)| {
            facing = turn(&facing, turn_direction);
            pos += facing * distance;
            (pos, facing)
        },
    );

    Some(abs(final_pos.x) + abs(final_pos.y))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut pos = IVec2::new(0, 0);
    let mut facing = IVec2::new(0, 1);
    let mut visited = Vec::from([pos]);

    input
        .trim()
        .split(", ")
        .filter_map(parse_instruction)
        .find_map(|(turn_direction, distance)| {
            facing = turn(&facing, turn_direction);
            let new_pos = pos + facing * distance;
            let intersection = find_intersection(&visited, &LineSegment::new(pos, new_pos));

            pos = new_pos;
            visited.push(pos);

            intersection.map(|inter| abs(inter.x) + abs(inter.y))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
    }
}
