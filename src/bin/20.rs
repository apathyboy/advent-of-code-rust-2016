use itertools::Itertools;

advent_of_code::solution!(20);

fn parse_line(line: &str) -> (usize, usize) {
    let (min_val, max_val) = line.split_once('-').unwrap();

    (min_val.parse().unwrap(), max_val.parse().unwrap())
}

pub fn part_one(input: &str) -> Option<usize> {
    let blacklist = input
        .lines()
        .map(parse_line)
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect::<Vec<_>>();

    let mut min_valid_ip = usize::MAX;

    for (min_val, max_val) in blacklist.iter() {
        if max_val + 1 < min_valid_ip
            && !blacklist
                .iter()
                .any(|(min, max)| max_val + 1 >= *min && max_val < max)
        {
            min_valid_ip = max_val + 1;
        } else if *min_val != 0
            && min_val - 1 < min_valid_ip
            && !blacklist
                .iter()
                .any(|(min, max)| min_val > min && min_val - 1 <= *max)
        {
            min_valid_ip = min_val - 1;
        }
    }

    Some(min_valid_ip)
}

pub fn part_two(input: &str) -> Option<usize> {
    let blacklist = input
        .lines()
        .map(parse_line)
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect::<Vec<_>>();

    let mut min_ip_range = usize::MAX;
    let mut max_ip_range = usize::MIN;

    let mut allowed_ips = 0;

    for (min_val, max_val) in blacklist.iter() {
        if max_val + 1 < min_ip_range
            && !blacklist
                .iter()
                .any(|(min, max)| max_val + 1 >= *min && max_val < max)
        {
            min_ip_range = max_val + 1;
        }

        if *min_val != 0
            && min_val - 1 > max_ip_range
            && !blacklist
                .iter()
                .any(|(min, max)| min_val > min && min_val - 1 <= *max)
        {
            max_ip_range = min_val - 1;
        }

        if min_ip_range != usize::MAX && max_ip_range != usize::MIN {
            allowed_ips += (max_ip_range - min_ip_range) + 1;

            min_ip_range = usize::MAX;
            max_ip_range = usize::MIN;
        }
    }

    if min_ip_range != usize::MAX && max_ip_range == usize::MIN {
        max_ip_range = if cfg!(test) { 9 } else { 4294967295 };
    }

    if min_ip_range != usize::MAX {
        allowed_ips += (max_ip_range - min_ip_range) + 1;
    }

    Some(allowed_ips)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
