use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(7);

lazy_static! {
    static ref IP_REGEX: Regex = Regex::new(r"(?:\[(.*?)\])|([^\[\]]+)").unwrap();
}

fn test_section(section: &str) -> bool {
    section
        .chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn supports_tls(ip: &str) -> bool {
    let mut supports_tls = false;

    // Iterate over all matches
    for cap in IP_REGEX.captures_iter(ip) {
        if let Some(hypernet) = cap.get(1) {
            // Segment inside brackets
            if test_section(hypernet.as_str()) {
                return false;
            } // Trim the brackets
        } else if let Some(supernet) = cap.get(2) {
            // Segment outside brackets
            if test_section(supernet.as_str()) {
                supports_tls = true;
            }
        }
    }

    supports_tls
}

fn find_potentials(block: &str) -> Vec<(char, char, char)> {
    block
        .chars()
        .tuple_windows()
        .filter_map(|(a, b, c)| {
            if a == c && a != b {
                Some((a, b, c))
            } else {
                None
            }
        })
        .collect()
}

fn supports_ssl(ip: &str) -> bool {
    let mut supernet_aba = Vec::new();
    let mut hypernet_bab = Vec::new();

    // Iterate over all matches
    for cap in IP_REGEX.captures_iter(ip) {
        if let Some(hypernet) = cap.get(1) {
            hypernet_bab.append(&mut find_potentials(hypernet.as_str()));
        } else if let Some(supernet) = cap.get(2) {
            supernet_aba.append(&mut find_potentials(supernet.as_str()));
        }
    }

    supernet_aba
        .iter()
        .any(|(a, b, _)| hypernet_bab.contains(&(*b, *a, *b)))
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter(|&line| supports_tls(line)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter(|&line| supports_ssl(line)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
