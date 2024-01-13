use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(11);

#[derive(Debug, PartialEq)]
enum ItemType {
    Generator,
    Microchip,
}

#[derive(Debug)]
struct Item {
    item_type: ItemType,
    name: String,
}

impl Item {
    fn new(item_type: &str, name: &str) -> Self {
        Self {
            item_type: match item_type {
                "generator" => ItemType::Generator,
                "microchip" => ItemType::Microchip,
                _ => panic!("Invalid item type"),
            },
            name: name.to_string(),
        }
    }

    fn label(&self) -> String {
        format!(
            "{}{}",
            &self.name[0..1].to_uppercase(),
            match self.item_type {
                ItemType::Generator => 'G',
                ItemType::Microchip => 'M',
            }
        )
    }
}

#[derive(Debug)]
struct Elevator {
    floor: usize,
    steps: usize,
}

impl Elevator {
    fn new() -> Self {
        Self { floor: 0, steps: 0 }
    }
}

lazy_static! {
    //static ref FLOOR_REGEX: Regex = Regex::new(r"(\w+) floor contains").unwrap();
    static ref ITEMS_REGEX: Regex =
        Regex::new(r"(?:a |and a )(\w+)(?:-compatible)? (generator|microchip)").unwrap();
}

fn parse_floor_contents(line: &str) -> Vec<Item> {
    //let floor = FLOOR_REGEX
    //    .captures(line)
    //    .and_then(|caps| caps.get(1))
    //    .map_or("", |m| m.as_str());
    //let floor = match floor {
    //    "first" => 1,
    //    "second" => 2,
    //    "third" => 3,
    //    "fourth" => 4,
    //    _ => panic!("Invalid floor"),
    //};

    let mut items = Vec::new();

    for item_cap in ITEMS_REGEX.captures_iter(line) {
        let element = item_cap.get(1).unwrap().as_str();
        let item_type = item_cap.get(2).unwrap().as_str();

        let item = Item::new(item_type, element);

        items.push(item);
    }

    items.sort_by(|a, b| b.label().cmp(&a.label()));

    println!("{:?}", &items);

    items
}

fn draw_scene(elevator: &Elevator, floors: &[Vec<Item>]) {
    let items = floors
        .iter()
        .flatten()
        .sorted_by(|&a, &b| a.label().cmp(&b.label()))
        .collect::<Vec<_>>();

    for (i, floor) in floors.iter().rev().enumerate() {
        let floor_num = floors.len() - i;
        print!("F{floor_num} ");

        let elevator_tile = if elevator.floor == floor_num - 1 {
            'E'
        } else {
            '.'
        };
        print!("{elevator_tile}  ");

        for item in items.iter() {
            if floor
                .iter()
                .find(|floor_item| floor_item.label() == item.label())
                .is_some()
            {
                print!("{} ", item.label());
            } else {
                print!(".  ");
            }
        }

        println!();
    }

    println!();
}

fn try_step(elevator: Elevator, floors: Vec<Vec<Item>>) -> Vec<(Elevator, Vec<Vec<Item>>)> {
    draw_scene(&elevator, &floors);

    if floors[0].is_empty() && floors[1].is_empty() && floors[2].is_empty() {
        return vec![(elevator, floors)];
    }

    let mut floors = floors;
    let mut elevator = elevator;

    match elevator.floor {
        0 => {
            println!("{:?}", &floors[0]);
            // try to move a pair up
            if floors[0].len() >= 2 {
                if floors[0][0].name == floors[0][1].name {
                    let item1 = floors[0].pop();
                    let item2 = floors[0].pop();

                    if item1.is_some() && item2.is_some() {
                        floors[1].push(item1.unwrap());
                        floors[1].push(item2.unwrap());

                        floors[1].sort_by(|a, b| b.label().cmp(&a.label()));

                        elevator.floor = 1;
                        elevator.steps += 1;

                        println!("moving");

                        return try_step(elevator, floors);
                    }
                }
            }

            if floors[0].len() >= 1 {
                let item1 = floors[0].pop();
                if item1.is_some() {
                    let item1 = item1.unwrap();
                    println!("moving {} to floor 2", &item1.label());
                    floors[1].push(item1);
                    floors[1].sort_by(|a, b| b.label().cmp(&a.label()));

                    elevator.floor = 1;
                    elevator.steps += 1;

                    return try_step(elevator, floors);
                }
            }
        }
        1 => {
            // try to move a pair up
            if floors[1].len() >= 2 {
                if floors[1][0].name == floors[1][1].name {
                    let item1 = floors[1].pop();
                    let item2 = floors[1].pop();

                    if item1.is_some() && item2.is_some() {
                        floors[2].push(item1.unwrap());
                        floors[2].push(item2.unwrap());

                        floors[2].sort_by(|a, b| b.label().cmp(&a.label()));

                        elevator.floor = 2;
                        elevator.steps += 1;

                        println!("moving");

                        return try_step(elevator, floors);
                    }
                }
            }
        }
        2 => {}
        3 => {}
        _ => panic!("Invalid floor"),
    }
    todo!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let floors: Vec<Vec<Item>> = input.lines().map(parse_floor_contents).collect();
    let elevator = Elevator::new();

    let completed_attempts = try_step(elevator, floors);

    completed_attempts
        .iter()
        .filter_map(|(elevator, floors)| {
            if !floors[0].is_empty() || !floors[1].is_empty() || !floors[2].is_empty() {
                return None;
            }

            Some(elevator.steps)
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
