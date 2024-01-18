use std::collections::VecDeque;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let elves = input.trim().parse::<usize>().unwrap();

    let mut remaining = (1..=elves).collect::<Vec<_>>();

    let mut skip = false;

    loop {
        let mut new_remaining = Vec::new();

        for i in remaining.into_iter() {
            if !skip {
                new_remaining.push(i);
            }

            skip = !skip;
        }

        remaining = new_remaining;

        if remaining.len() == 1 {
            break;
        }
    }

    Some(remaining[0])
}

pub fn part_two(input: &str) -> Option<usize> {
    let elves = input.trim().parse::<usize>().unwrap();

    let mut remaining = (1..=elves).collect::<VecDeque<_>>();

    loop {
        let idx = remaining.len() / 2;

        remaining.remove(idx);

        if remaining.len() == 1 {
            break;
        }

        let old_front = remaining.pop_front().unwrap();
        remaining.push_back(old_front);
    }

    Some(remaining[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
