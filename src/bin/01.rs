use glam::IVec2;
use num_traits::abs;
use num_traits::signum;

advent_of_code::solution!(1);

enum TurnDirection {
    Left,
    Right,
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

pub fn part_one(input: &str) -> Option<i32> {
    let mut pos = IVec2::new(0, 0);
    let mut facing = IVec2::new(0, 1);

    for (turn_direction, distance) in input.trim().split(", ").map(|instruction| {
        let turn_direction = match &instruction[0..1] {
            "L" => TurnDirection::Left,
            "R" => TurnDirection::Right,
            _ => panic!("Invalid instruction"),
        };

        let distance = instruction[1..].parse::<i32>().unwrap();

        (turn_direction, distance)
    }) {
        facing = turn(&facing, turn_direction);
        pos += facing * distance;
    }

    Some(abs(pos.x) + abs(pos.y))
}

pub fn part_two(input: &str) -> Option<i32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
