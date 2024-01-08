use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: Vec<char>,
}

impl Room {
    fn new(encrypted_name: &str, sector_id: u32, checksum: &str) -> Self {
        Self {
            encrypted_name: encrypted_name.to_string(),
            sector_id,
            checksum: checksum.chars().collect::<Vec<_>>(),
        }
    }

    fn is_real(&self) -> bool {
        let mut freq_map = HashMap::new();

        for c in self.encrypted_name.chars() {
            if c != '-' {
                *freq_map.entry(c).or_insert(0) += 1;
            }
        }

        let mut freq_vec: Vec<(char, i32)> = freq_map.into_iter().collect();

        freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        freq_vec
            .iter()
            .zip(&self.checksum)
            .take(5)
            .all(|((a, _), b)| *a == *b)
    }

    fn decrypt(&self) -> String {
        let shift = (self.sector_id % 26) as u8;

        self.encrypted_name
            .chars()
            .map(|c| match c {
                'a'..='z' => ((((c as u8 - b'a') + shift) % 26) + b'a') as char,
                _ => ' ',
            })
            .collect()
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

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().filter_map(parse_room).find_map(|room| {
        if room.is_real() && room.decrypt().contains("pole") {
            Some(room.sector_id)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_room() {
        let result = parse_room("aaaaa-bbb-z-y-x-123[abxyz]");
        let room = Room::new("aaaaa-bbb-z-y-x", 123, "abxyz");
        assert_eq!(result, Some(room));
    }

    #[test]
    fn test_decrypt_room() {
        let room = Room::new("qzmt-zixmtkozy-ivhz", 343, "abxyz");
        assert_eq!(room.decrypt(), "very encrypted name".to_string());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1514));
    }
}
