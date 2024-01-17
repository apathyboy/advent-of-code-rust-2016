use itertools::Itertools;

advent_of_code::solution!(16);

fn checksum(input: &str) -> String {
    let mut checksum = input
        .chars()
        .tuples()
        .map(|(a, b)| if a == b { '1' } else { '0' })
        .collect::<String>();

    while checksum.len() % 2 == 0 {
        checksum = checksum
            .chars()
            .tuples()
            .map(|(a, b)| if a == b { '1' } else { '0' })
            .collect::<String>();
    }

    checksum
}

fn step(input: &str) -> String {
    let opposite = input
        .chars()
        .rev()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>();

    format!("{input}0{opposite}")
}

fn fill_disk(input: &str, disk_size: usize) -> String {
    let mut disk_contents = step(input);

    while disk_contents.len() < disk_size {
        disk_contents = step(&disk_contents);
    }

    checksum(&disk_contents[0..disk_size])
}

pub fn part_one(input: &str) -> Option<String> {
    Some(fill_disk(input.trim(), 272))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(fill_disk(input.trim(), 35651584))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_disk() {
        let result = fill_disk("10000", 20);
        assert_eq!(result, String::from("01100"));
    }

    #[test]
    fn test_checksum() {
        let result = checksum("110010110100");
        assert_eq!(result, String::from("100"));
    }

    #[test]
    fn test_step() {
        let result = step("1");
        assert_eq!(result, String::from("100"));

        let result = step("0");
        assert_eq!(result, String::from("001"));

        let result = step("11111");
        assert_eq!(result, String::from("11111000000"));

        let result = step("111100001010");
        assert_eq!(result, String::from("1111000010100101011110000"));
    }
}
