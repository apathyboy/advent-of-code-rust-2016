use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq)]
struct Room {
    encrypted_name: Vec<char>,
    sector_id: u32,
    checksum: Vec<char>,
}

impl Room {
    fn new(encrypted_name: &str, sector_id: u32, checksum: &str) -> Self {
        let mut encrypted_name = encrypted_name
            .chars()
            .filter(|c| *c != '-')
            .collect::<Vec<_>>();

        encrypted_name.sort();

        Self {
            encrypted_name,
            sector_id,
            checksum: checksum.chars().collect::<Vec<_>>(),
        }
    }

    fn is_real(&self) -> bool {
        let mut freq_map = HashMap::new();

        for &c in &self.encrypted_name {
            *freq_map.entry(c).or_insert(0) += 1;
        }

        let mut freq_vec: Vec<(char, i32)> = freq_map.into_iter().collect();

        freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        freq_vec
            .iter()
            .zip(&self.checksum)
            .take(5)
            .all(|((a, _), b)| *a == *b)
    }
}

fn parse_room(line: &str) -> Option<Room> {
    let checksum = &line[line.len() - 6..line.len() - 1];
    let sector_id = &line[line.len() - 10..line.len() - 7]
        .parse::<u32>()
        .unwrap();
    let encrypted_name = &line[..line.len() - 11];

    Some(Room::new(encrypted_name, *sector_id, checksum))
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum_of_sectors = input
        .lines()
        .filter_map(parse_room)
        .filter_map(|room| {
            if room.is_real() {
                Some(room.sector_id)
            } else {
                None
            }
        })
        .sum();

    Some(sum_of_sectors)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_parse_room() {
        let result = parse_room("aaaaa-bbb-z-y-x-123[abxyz]");
        let room = Room {
            encrypted_name: "aaaaabbbxyz".chars().collect_vec(),
            sector_id: 123,
            checksum: "abxyz".chars().collect_vec(),
        };
        assert_eq!(result, Some(room));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1514));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
