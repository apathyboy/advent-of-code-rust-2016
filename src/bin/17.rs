use glam::IVec2;
use pathfinding::prelude::{bfs, dfs_reach};

advent_of_code::solution!(17);

fn successors(room: &(Vec<char>, IVec2), passcode: &str) -> Vec<(Vec<char>, IVec2)> {
    let doors_secret = format!("{}{}", passcode, room.0.iter().collect::<String>());
    let doors_hash = advent_of_code::make_hash(&doors_secret)
        .chars()
        .take(4)
        .collect::<Vec<_>>();
    let open_states = ['b', 'c', 'd', 'e', 'f'];

    let mut successors = Vec::new();

    if room.1 == IVec2::new(3, 3) {
        return successors;
    }

    // up
    if room.1.y != 0 && open_states.contains(&doors_hash[0]) {
        let mut updated_path = room.0.clone();
        updated_path.push('U');

        successors.push((updated_path, room.1 + IVec2::new(0, -1)));
    }

    // down
    if room.1.y != 3 && open_states.contains(&doors_hash[1]) {
        let mut updated_path = room.0.clone();
        updated_path.push('D');

        successors.push((updated_path, room.1 + IVec2::new(0, 1)));
    }

    // left
    if room.1.x != 0 && open_states.contains(&doors_hash[2]) {
        let mut updated_path = room.0.clone();
        updated_path.push('L');

        successors.push((updated_path, room.1 + IVec2::new(-1, 0)));
    }

    // right
    if room.1.x != 3 && open_states.contains(&doors_hash[3]) {
        let mut updated_path = room.0.clone();
        updated_path.push('R');

        successors.push((updated_path, room.1 + IVec2::new(1, 0)));
    }

    successors
}

fn shortest_path(start: IVec2, goal: IVec2, passcode: &str) -> Option<String> {
    let start: (Vec<char>, IVec2) = (Vec::new(), start);

    let result = bfs(&start, |p| successors(p, passcode), |p| p.1 == goal);

    let path = result
        .as_ref()
        .unwrap()
        .last()
        .unwrap()
        .0
        .iter()
        .collect::<String>();

    Some(path)
}

fn longest_path_length(start: IVec2, goal: IVec2, passcode: &str) -> Option<usize> {
    let start: (Vec<char>, IVec2) = (Vec::new(), start);

    dfs_reach(start, |p| successors(p, passcode))
        .filter_map(
            |(path, pos)| {
                if pos == goal {
                    Some(path.len())
                } else {
                    None
                }
            },
        )
        .max()
}

pub fn part_one(input: &str) -> Option<String> {
    let passcode = input.trim();

    shortest_path(IVec2::new(0, 0), IVec2::new(3, 3), passcode)
}

pub fn part_two(input: &str) -> Option<usize> {
    let passcode = input.trim();

    longest_path_length(IVec2::new(0, 0), IVec2::new(3, 3), passcode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ihgpwlah", "DDRRRD")]
    #[case("kglvqrro", "DDUDRLRRUDRD")]
    #[case("ulqzkmiv", "DRURDRUDDLLDLUURRDULRLDUUDDDRR")]
    fn test_part_one(#[case] input: &str, #[case] expected: &str) {
        let result = part_one(input);
        assert_eq!(result, Some(String::from(expected)));
    }

    #[rstest]
    #[case("ihgpwlah", 370)]
    #[case("kglvqrro", 492)]
    #[case("ulqzkmiv", 830)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
