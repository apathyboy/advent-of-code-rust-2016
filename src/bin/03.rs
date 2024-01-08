use itertools::Itertools;

advent_of_code::solution!(3);

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let triangles = input
        .lines()
        .filter(|line| {
            let mut vals = parse_line(line);
            vals.sort();

            vals[0] + vals[1] > vals[2]
        })
        .count();

    Some(triangles as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let triangles = input
        .lines()
        .tuples()
        .flat_map(|(a, b, c)| {
            let vals1 = parse_line(a);
            let vals2 = parse_line(b);
            let vals3 = parse_line(c);

            let mut vals1 = vec![vals1[0], vals2[0], vals3[0]];
            vals1.sort();

            let mut vals2 = vec![vals1[1], vals2[1], vals3[1]];
            vals2.sort();

            let mut vals3 = vec![vals1[2], vals2[2], vals3[2]];
            vals3.sort();

            vec![vals1, vals2, vals3]
        })
        .filter(|vals| vals[0] + vals[1] > vals[2])
        .count();

    Some(triangles as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
