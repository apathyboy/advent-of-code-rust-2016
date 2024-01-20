use advent_of_code::PrototypeComputer;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i32> {
    let program = input.lines().collect::<Vec<_>>();

    let mut computer = PrototypeComputer::new();

    computer.run_program(&program);

    computer.get_register('a')
}

pub fn part_two(input: &str) -> Option<i32> {
    let program = input.lines().collect::<Vec<_>>();

    let mut computer = PrototypeComputer::new();

    computer.set_register('c', 1);

    computer.run_program(&program);

    computer.get_register('a')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
