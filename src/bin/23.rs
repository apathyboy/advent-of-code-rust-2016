use advent_of_code::PrototypeComputer;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<i32> {
    let program = input.lines().collect::<Vec<_>>();

    let mut computer = PrototypeComputer::new();

    computer.set_register('a', 7);

    computer.run_program(&program);

    computer.get_register('a')
}

pub fn part_two(input: &str) -> Option<u64> {
    let computer = PrototypeComputer::new();
    let program = computer.parse_program(&input.lines().collect::<Vec<_>>());

    let num: u64 = program
        .iter()
        .filter_map(|i| {
            if i.name != "cpy" && i.name != "jnz" {
                return None;
            }

            let val = i.arguments[0].parse::<u64>();
            if val.is_err() {
                return None;
            }

            let val = val.unwrap();

            if val > 90 {
                Some(val)
            } else {
                None
            }
        })
        .product();

    let factorial: u64 = (1..=12).product();

    Some(num + factorial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let program = input.lines().collect::<Vec<_>>();

        let mut computer = PrototypeComputer::new();

        computer.run_program(&program);

        let result = computer.get_register('a');
        assert_eq!(result, Some(3));
    }
}
