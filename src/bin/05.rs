use advent_of_code::{make_hash, make_secret};

advent_of_code::solution!(5);

pub fn is_valid_hash(input: &str, precision: usize) -> bool {
    input.starts_with(&"0".repeat(precision))
}

pub fn part_one(input: &str) -> Option<String> {
    let door_id = input.trim();

    let mut password = Vec::new();

    for i in 0.. {
        let secret = make_secret(door_id, i);
        let hash = make_hash(&secret);

        if is_valid_hash(&hash, 5) {
            password.push(hash.chars().nth(5).unwrap());

            if password.len() == 8 {
                break;
            }
        }
    }

    Some(password.into_iter().collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let door_id = input.trim();

    let password = &mut [None; 8];

    for i in 0.. {
        let secret = make_secret(door_id, i);
        let hash = make_hash(&secret);
        let idx = hash.chars().nth(5).unwrap();

        if !is_valid_hash(&hash, 5) || !idx.is_ascii_digit() {
            continue;
        }

        let idx = idx.to_digit(10).unwrap() as usize;

        if idx > 7 || password[idx].is_some() {
            continue;
        }

        password[idx] = Some(hash.chars().nth(6).unwrap());

        if password.iter().all(|c| c.is_some()) {
            break;
        }
    }

    Some(password.iter().flatten().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("18f47a30".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("05ace8e3".to_string()));
    }
}
