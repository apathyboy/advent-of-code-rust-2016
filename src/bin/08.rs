use std::collections::HashMap;

use glam::IVec2;

#[derive(Debug, Copy, Clone, PartialEq)]
enum PixelState {
    On,
    Off,
}

#[derive(Debug)]
enum Instruction {
    Rect { width: i32, height: i32 },
    RotateRow { row: i32, shift_amount: i32 },
    RotateColumn { column: i32, shift_amount: i32 },
}

advent_of_code::solution!(8);

fn create_screen(width: i32, height: i32) -> HashMap<IVec2, PixelState> {
    let mut screen: HashMap<IVec2, PixelState> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            screen.insert(IVec2::new(x, y), PixelState::Off);
        }
    }

    screen
}

fn fix_screen(
    screen_width: i32,
    screen_height: i32,
    instructions: &str,
) -> HashMap<IVec2, PixelState> {
    let mut screen = create_screen(screen_width, screen_height);

    for instruction in instructions.lines().filter_map(parse_instruction) {
        match instruction {
            Instruction::Rect { width, height } => {
                for x in 0..width {
                    for y in 0..height {
                        *screen.get_mut(&IVec2::new(x, y)).unwrap() = PixelState::On;
                    }
                }
            }
            Instruction::RotateColumn {
                column,
                shift_amount,
            } => {
                let mut new_column = Vec::new();

                for y in 0..screen_height {
                    let cur = *screen.get(&IVec2::new(column, y)).unwrap();
                    new_column.push((IVec2::new(column, (y + shift_amount) % screen_height), cur));
                }

                for (pos, val) in new_column {
                    *screen.get_mut(&pos).unwrap() = val;
                }
            }
            Instruction::RotateRow { row, shift_amount } => {
                let mut new_row = Vec::new();

                for x in 0..screen_width {
                    let cur = *screen.get(&IVec2::new(x, row)).unwrap();
                    new_row.push((IVec2::new((x + shift_amount) % screen_width, row), cur));
                }

                for (pos, val) in new_row {
                    *screen.get_mut(&pos).unwrap() = val;
                }
            }
        }
    }

    screen
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let (name, rest) = line.split_once(' ').unwrap();

    match name {
        "rect" => {
            let (width, height) = rest.trim().split_once('x').unwrap();

            Some(Instruction::Rect {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
            })
        }
        "rotate" => {
            let (rotation_type, rest) = rest.trim().split_once(' ').unwrap();

            match rotation_type {
                "row" => {
                    let (row, shift_amount) = rest[2..].split_once(" by ").unwrap();

                    Some(Instruction::RotateRow {
                        row: row.parse().unwrap(),
                        shift_amount: shift_amount.parse().unwrap(),
                    })
                }
                "column" => {
                    let (column, shift_amount) = rest[2..].split_once(" by ").unwrap();

                    Some(Instruction::RotateColumn {
                        column: column.parse().unwrap(),
                        shift_amount: shift_amount.parse().unwrap(),
                    })
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn count_on_pixels(screen: &HashMap<IVec2, PixelState>) -> usize {
    screen.iter().filter(|(_, &b)| b == PixelState::On).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let screen_width = 50;
    let screen_height = 6;

    let screen = fix_screen(screen_width, screen_height, input);

    Some(count_on_pixels(&screen))
}

pub fn part_two(input: &str) -> Option<String> {
    let screen_width = 50;
    let screen_height = 6;

    let screen = fix_screen(screen_width, screen_height, input);

    Some(
        (0..screen_height)
            .map(|y| {
                (0..screen_width)
                    .map(|x| match *screen.get(&IVec2::new(x, y)).unwrap() {
                        PixelState::On => '#',
                        PixelState::Off => ' ',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let screen_width = 7;
        let screen_height = 3;

        let screen = fix_screen(
            screen_width,
            screen_height,
            &advent_of_code::template::read_file("examples", DAY),
        );

        let on_pixels = count_on_pixels(&screen);

        assert_eq!(on_pixels, 6);
    }
}
