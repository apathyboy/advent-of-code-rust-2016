use std::collections::HashMap;

advent_of_code::solution!(12);

fn run_program(registers: &mut HashMap<&str, i32>, program: &[&str]) {
    let mut ip = 0;

    loop {
        if ip >= program.len() {
            break;
        }

        let instruction = program[ip];
        let (name, rest) = instruction.split_once(' ').unwrap();

        match name {
            "cpy" => {
                let (source, dest) = rest.split_once(' ').unwrap();

                let source_val = if registers.contains_key(source) {
                    *registers.get(source).unwrap()
                } else {
                    source.parse::<i32>().unwrap()
                };

                *registers.get_mut(dest).unwrap() = source_val;

                ip += 1;
            }
            "inc" => {
                *registers.get_mut(rest).unwrap() += 1;

                ip += 1;
            }
            "dec" => {
                *registers.get_mut(rest).unwrap() -= 1;

                ip += 1;
            }
            "jnz" => {
                let (source, dest) = rest.split_once(' ').unwrap();

                let x_val = if registers.contains_key(source) {
                    *registers.get(source).unwrap()
                } else {
                    source.parse::<i32>().unwrap()
                };

                if x_val != 0 {
                    let skip = dest.parse::<i32>().unwrap();

                    ip = if skip < 0 {
                        let skip = (-skip) as usize;
                        ip.checked_sub(skip).unwrap()
                    } else {
                        ip.checked_add(skip as usize).unwrap()
                    };
                } else {
                    ip += 1;
                }
            }
            _ => panic!("Invalid instruction {name}"),
        };
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut registers: HashMap<&str, i32> = HashMap::from([("a", 0), ("b", 0), ("c", 0), ("d", 0)]);
    let program = input.lines().collect::<Vec<_>>();

    run_program(&mut registers, &program);

    registers.get(&"a").copied()
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut registers: HashMap<&str, i32> = HashMap::from([("a", 0), ("b", 0), ("c", 1), ("d", 0)]);
    let program = input.lines().collect::<Vec<_>>();

    run_program(&mut registers, &program);

    registers.get(&"a").copied()
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
