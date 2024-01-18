advent_of_code::solution!(18);

fn next_row(row: &[char]) -> Vec<char> {
    let mut next = Vec::new();

    for i in 0..row.len() {
        let left = if i > 0 { row[i - 1] } else { '.' };
        let center = row[i];
        let right = if i < row.len() - 1 { row[i + 1] } else { '.' };

        let is_only_left = left == '^' && center == '.' && right == '.';
        let is_only_right = left == '.' && center == '.' && right == '^';
        let is_left_center = left == '^' && center == '^' && right == '.';
        let is_right_center = left == '.' && center == '^' && right == '^';
        let is_trap = is_left_center || is_right_center || is_only_left || is_only_right;

        let tile = if is_trap { '^' } else { '.' };

        next.push(tile);
    }

    next
}

fn determine_rows(mut row: Vec<char>, row_count: usize) -> usize {
    let mut safe_tile_count = 0;

    for _ in 0..row_count {
        safe_tile_count += row.iter().filter(|&tile| *tile == '.').count();
        row = next_row(&row);
    }

    safe_tile_count
}

pub fn part_one(input: &str) -> Option<usize> {
    let row: Vec<char> = input.trim().chars().collect();

    Some(determine_rows(row, 40))
}

pub fn part_two(input: &str) -> Option<usize> {
    let row: Vec<char> = input.trim().chars().collect();

    Some(determine_rows(row, 400000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_rows() {
        let row = vec!['.', '^', '^', '.', '^', '.', '^', '^', '^', '^'];
        let result = determine_rows(row, 10);

        assert_eq!(result, 38);
    }
}
