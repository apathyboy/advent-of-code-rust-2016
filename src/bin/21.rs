use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(21);

lazy_static! {
    static ref SWAP_POSITION_REGEX: Regex =
        Regex::new(r"swap position ([0-9]+) with position ([0-9]+)").unwrap();
    static ref SWAP_LETTER_REGEX: Regex =
        Regex::new(r"swap letter ([a-z]) with letter ([a-z])").unwrap();
    static ref REVERSE_POSITIONS_REGEX: Regex =
        Regex::new(r"reverse positions ([0-9]+) through ([0-9]+)").unwrap();
    static ref ROTATE_LEFT_REGEX: Regex = Regex::new(r"rotate left ([0-9]+)").unwrap();
    static ref ROTATE_RIGHT_REGEX: Regex = Regex::new(r"rotate right ([0-9]+)").unwrap();
    static ref MOVE_POSITION_REGEX: Regex =
        Regex::new(r"move position ([0-9]+) to position ([0-9]+)").unwrap();
    static ref ROTATE_RELATIVE_REGEX: Regex =
        Regex::new(r"rotate based on position of letter ([a-z])").unwrap();
}

fn scramble_password(operations: &str, password: &str) -> String {
    let mut password = password.to_string();

    for operation in operations.lines() {
        if operation.starts_with("swap position") {
            let (pos1, pos2) = SWAP_POSITION_REGEX
                .captures(operation)
                .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
                .unwrap();

            password = swap_position(&password, pos1, pos2);
        } else if operation.starts_with("swap letter") {
            let (letter1, letter2) = SWAP_LETTER_REGEX
                .captures(operation)
                .map(|cap| {
                    (
                        cap[1].chars().next().unwrap(),
                        cap[2].chars().next().unwrap(),
                    )
                })
                .unwrap();

            password = swap_letter(&password, letter1, letter2);
        } else if operation.starts_with("reverse positions") {
            let (start_pos, end_pos) = REVERSE_POSITIONS_REGEX
                .captures(operation)
                .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
                .unwrap();

            password = reverse_positions(&password, start_pos, end_pos);
        } else if operation.starts_with("rotate left") {
            let shift_amount = ROTATE_LEFT_REGEX
                .captures(operation)
                .map(|cap| cap[1].parse().unwrap())
                .unwrap();

            password = rotate_left(&password, shift_amount);
        } else if operation.starts_with("rotate right") {
            let shift_amount = ROTATE_RIGHT_REGEX
                .captures(operation)
                .map(|cap| cap[1].parse().unwrap())
                .unwrap();

            password = rotate_right(&password, shift_amount);
        } else if operation.starts_with("move position") {
            let (remove_pos, insert_pos) = MOVE_POSITION_REGEX
                .captures(operation)
                .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
                .unwrap();

            password = move_position(&password, remove_pos, insert_pos);
        } else if operation.starts_with("rotate based") {
            let letter = ROTATE_RELATIVE_REGEX
                .captures(operation)
                .map(|cap| cap[1].chars().next().unwrap())
                .unwrap();

            password = rotate_relative(&password, letter);
        }
    }

    password
}

fn rotate_relative(input: &str, letter: char) -> String {
    let mut rotate_amount = 1;
    let letter_pos = input.chars().position(|c| c == letter).unwrap();
    rotate_amount += letter_pos + if letter_pos >= 4 { 1 } else { 0 };

    rotate_right(input, rotate_amount)
}

fn move_position(input: &str, remove_pos: usize, insert_pos: usize) -> String {
    let mut input = input.chars().collect::<Vec<_>>();

    let val = input.remove(remove_pos);

    input.insert(insert_pos, val);

    input.iter().collect()
}

fn swap_position(input: &str, pos1: usize, pos2: usize) -> String {
    let mut input = input.chars().collect::<Vec<_>>();

    input.swap(pos1, pos2);

    input.iter().collect()
}

fn swap_letter(input: &str, letter1: char, letter2: char) -> String {
    let mut input = input.chars().collect::<Vec<_>>();

    let letter1_pos = input.iter().position(|&c| c == letter1).unwrap();
    let letter2_pos = input.iter().position(|&c| c == letter2).unwrap();

    input[letter1_pos] = letter2;
    input[letter2_pos] = letter1;

    input.iter().collect()
}

fn reverse_positions(input: &str, start_pos: usize, end_pos: usize) -> String {
    let mut input = input.chars().collect::<Vec<_>>();

    input[start_pos..=end_pos].reverse();

    input.iter().collect()
}

fn rotate_right(input: &str, shift_amount: usize) -> String {
    let mut input = input.chars().collect::<Vec<_>>();
    let effective_shift_amount = shift_amount.rem_euclid(input.len());

    input.rotate_right(effective_shift_amount);

    input.iter().collect()
}

fn rotate_left(input: &str, shift_amount: usize) -> String {
    let mut input = input.chars().collect::<Vec<_>>();
    let effective_shift_amount = shift_amount.rem_euclid(input.len());

    input.rotate_left(effective_shift_amount);

    input.iter().collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let password = if cfg!(test) { "abcde" } else { "abcdefgh" };

    Some(scramble_password(input, password))
}

pub fn part_two(input: &str) -> Option<String> {
    let password = if cfg!(test) { "decab" } else { "fbgdceah" };

    password
        .chars()
        .permutations(password.len())
        .map(|v| v.iter().collect::<String>())
        .find(|s| scramble_password(input, s) == password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abdec", 'b', "ecabd")]
    #[case("ecabd", 'd', "decab")]
    fn test_rotate_relative(#[case] input: &str, #[case] letter: char, #[case] expected: &str) {
        let result = rotate_relative(input, letter);
        assert_eq!(result, String::from(expected));
    }

    #[test]
    fn test_move_position() {
        let result = move_position("bcdea", 1, 4);
        assert_eq!(result, String::from("bdeac"));
    }

    #[test]
    fn test_rotate_right() {
        let result = rotate_right("bcdea", 1);
        assert_eq!(result, String::from("abcde"));
    }

    #[test]
    fn test_rotate_left() {
        let result = rotate_left("abcde", 1);
        assert_eq!(result, String::from("bcdea"));
    }

    #[test]
    fn test_reverse_positions() {
        let result = reverse_positions("edcba", 0, 4);
        assert_eq!(result, String::from("abcde"));
    }

    #[test]
    fn test_swap_letter() {
        let result = swap_letter("ebcda", 'd', 'b');
        assert_eq!(result, String::from("edcba"));
    }

    #[test]
    fn test_swap_position() {
        let result = swap_position("abcde", 4, 0);
        assert_eq!(result, String::from("ebcda"));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("decab")));
    }
}
