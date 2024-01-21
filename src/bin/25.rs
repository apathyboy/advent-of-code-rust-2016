use advent_of_code::PrototypeComputer;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<i32> {
    let program = input.lines().collect::<Vec<_>>();

    for i in 0.. {
        let mut computer = PrototypeComputer::new();
        computer.load_program(&program);
        computer.set_register('a', i);

        let mut expected = 0;
        let mut is_valid = true;

        for _ in 0..10 {
            let result = computer.run_to_next_output();

            if result != expected {
                is_valid = false;
                break;
            }

            expected = (expected + 1).rem_euclid(2);
        }

        if is_valid {
            return Some(i);
        }
    }

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
