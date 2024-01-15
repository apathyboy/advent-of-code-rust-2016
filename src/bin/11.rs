advent_of_code::solution!(11);

fn parse_floor_contents(line: &str) -> usize {
    let mut items = 0;

    if !line.contains("nothing") {
        items = line.chars().filter(|c| *c == '.' || *c == ',').count();

        if items == 1 && line.contains(" and ") {
            items += 1;
        }
    }

    items
}

pub fn part_one(input: &str) -> Option<usize> {
    let floors: Vec<usize> = input.lines().map(parse_floor_contents).collect();

    let result = (1..4)
        .map(|x| 2 * floors.iter().take(x).sum::<usize>() - 3)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut floors: Vec<usize> = input.lines().map(parse_floor_contents).collect();

    floors[0] += 4;

    let result = (1..4)
        .map(|x| 2 * floors.iter().take(x).sum::<usize>() - 3)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
