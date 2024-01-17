use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(15);

struct Disc {
    //   id: usize,
    positions: usize,
    start_position: usize,
}

lazy_static! {
    static ref DISC_REGEX: Regex = Regex::new(
        r"Disc #([0-9]+) has ([0-9]+) positions; at time=0, it is at position ([0-9]+)."
    )
    .unwrap();
}

fn parse_disc(line: &str) -> Option<Disc> {
    DISC_REGEX.captures(line).map(|cap| {
        //let id = cap[1].parse().unwrap();
        let positions = cap[2].parse().unwrap();
        let start_position = cap[3].parse().unwrap();

        Disc {
            //    id,
            positions,
            start_position,
        }
    })
}

fn play_game(discs: &[Disc], start_time: usize) -> bool {
    for (time, disc) in (start_time + 1..).zip(discs) {
        if (time + disc.start_position) % disc.positions != 0 {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let discs: Vec<Disc> = input.lines().filter_map(parse_disc).collect();

    (0..).find(|start_time| play_game(&discs, *start_time))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut discs: Vec<Disc> = input.lines().filter_map(parse_disc).collect();

    discs.push(Disc {
        positions: 11,
        start_position: 0,
    });

    (0..).find(|start_time| play_game(&discs, *start_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
