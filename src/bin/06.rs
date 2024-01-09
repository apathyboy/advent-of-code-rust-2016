use std::collections::HashMap;

advent_of_code::solution!(6);

fn transpose(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

fn find_least_frequent(line: &[char]) -> Option<char> {
    let mut freq_map = HashMap::new();

    for &c in line.iter() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let mut freq_vec: Vec<(char, i32)> = freq_map.into_iter().collect();

    freq_vec.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    freq_vec.iter().map(|(c, _)| *c).next()
}

fn find_most_frequent(line: &[char]) -> Option<char> {
    let mut freq_map = HashMap::new();

    for &c in line.iter() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let mut freq_vec: Vec<(char, i32)> = freq_map.into_iter().collect();

    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    freq_vec.iter().map(|(c, _)| *c).next()
}

pub fn part_one(input: &str) -> Option<String> {
    let recording = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Some(
        transpose(&recording)
            .iter()
            .filter_map(|line| find_most_frequent(line))
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let recording = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Some(
        transpose(&recording)
            .iter()
            .filter_map(|line| find_least_frequent(line))
            .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("easter".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("advent".to_string()));
    }
}
