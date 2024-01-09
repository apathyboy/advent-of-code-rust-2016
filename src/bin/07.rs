use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(7);

fn test_section(section: &str) -> bool {
    for (a, b, c, d) in section.chars().tuple_windows() {
        if a == d && b == c && a != b {
            return true;
        }
    }

    false
}

fn supports_tls(ip: &str) -> bool {
    // Regex pattern to match text inside and outside brackets
    let re = Regex::new(r"(?:\[(.*?)\])|([^\[\]]+)").unwrap();

    let mut supports_tls = false;

    // Iterate over all matches
    for cap in re.captures_iter(ip) {
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
    let mut potentials = Vec::new();

    for (a, b, c) in block.chars().tuple_windows() {
        if a == c && a != b {
            potentials.push((a, b, c));
        }
    }

    potentials
}

fn supports_ssl(ip: &str) -> bool {
    // Regex pattern to match text inside and outside brackets
    let re = Regex::new(r"(?:\[(.*?)\])|([^\[\]]+)").unwrap();

    let mut supernet_aba = Vec::new();
    let mut hypernet_bab = Vec::new();

    // Iterate over all matches
    for cap in re.captures_iter(ip) {
        if let Some(hypernet) = cap.get(1) {
            hypernet_bab.append(&mut find_potentials(hypernet.as_str()));
        } else if let Some(supernet) = cap.get(2) {
            supernet_aba.append(&mut find_potentials(supernet.as_str()));
        }
    }

    for (a, b, _) in supernet_aba {
        if hypernet_bab.contains(&(b, a, b)) {
            return true;
        }
    }

    false
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
