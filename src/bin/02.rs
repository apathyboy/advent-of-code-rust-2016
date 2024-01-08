use glam::IVec2;
use std::collections::HashMap;

advent_of_code::solution!(2);

const KEYPAD1: &[(IVec2, char)] = &[
    // Define the keypad layout here
    (IVec2::new(0, 0), '1'),
    (IVec2::new(1, 0), '2'),
    (IVec2::new(2, 0), '3'),
    (IVec2::new(0, 1), '4'),
    (IVec2::new(1, 1), '5'),
    (IVec2::new(2, 1), '6'),
    (IVec2::new(0, 2), '7'),
    (IVec2::new(1, 2), '8'),
    (IVec2::new(2, 2), '9'),
];

const KEYPAD2: &[(IVec2, char)] = &[
    // Define the keypad layout here
    (IVec2::new(2, 0), '1'),
    (IVec2::new(1, 1), '2'),
    (IVec2::new(2, 1), '3'),
    (IVec2::new(3, 1), '4'),
    (IVec2::new(0, 2), '5'),
    (IVec2::new(1, 2), '6'),
    (IVec2::new(2, 2), '7'),
    (IVec2::new(3, 2), '8'),
    (IVec2::new(4, 2), '9'),
    (IVec2::new(1, 3), 'A'),
    (IVec2::new(2, 3), 'B'),
    (IVec2::new(3, 3), 'C'),
    (IVec2::new(2, 4), 'D'),
];

const DIRECTIONS: &[(char, IVec2)] = &[
    // Define the directions here
    ('U', IVec2::new(0, -1)),
    ('D', IVec2::new(0, 1)),
    ('L', IVec2::new(-1, 0)),
    ('R', IVec2::new(1, 0)),
];

fn find_code(keypad: &HashMap<IVec2, char>, initial_button: &IVec2, input: &str) -> String {
    let directions: HashMap<_, _> = DIRECTIONS.iter().cloned().collect();

    let mut current_button = initial_button.clone();
    let mut bathroom_code = Vec::new();

    for line in input.lines() {
        for instruction in line.chars() {
            let next_button = *directions.get(&instruction).unwrap() + current_button;

            if keypad.contains_key(&next_button) {
                current_button = next_button;
            }
        }

        bathroom_code.push(keypad.get(&current_button).unwrap());
    }

    bathroom_code.into_iter().collect::<String>()
}

pub fn part_one(input: &str) -> Option<String> {
    let keypad: HashMap<_, _> = KEYPAD1.iter().cloned().collect();

    Some(find_code(&keypad, &IVec2::new(1, 1), input))
}

pub fn part_two(input: &str) -> Option<String> {
    let keypad: HashMap<_, _> = KEYPAD2.iter().cloned().collect();

    Some(find_code(&keypad, &IVec2::new(0, 2), input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("1985".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5DB3".to_string()));
    }
}
