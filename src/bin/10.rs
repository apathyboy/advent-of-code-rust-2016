use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Bot {
    id: String,
    low_output: String,
    high_output: String,
    chips: Vec<usize>,
    is_responsible: bool,
}

impl Bot {
    fn new(id: &str, low_output: &str, high_output: &str) -> Self {
        Self {
            id: String::from(id),
            low_output: String::from(low_output),
            high_output: String::from(high_output),
            chips: Vec::new(),
            is_responsible: false,
        }
    }
}

fn parse_bot_definition(line: &str) -> Option<Bot> {
    if !line.starts_with("bot") {
        return None;
    }

    let re = Regex::new(r"(bot [0-9]+) gives low to (.* [0-9]+) and high to (.* [0-9]+)").unwrap();

    re.captures(line).map(|cap| {
        let bot_id = cap.get(1).map(|bot_id| bot_id.as_str()).unwrap();
        let low_id = cap.get(2).map(|low_id| low_id.as_str()).unwrap();
        let high_id = cap.get(3).map(|high_id| high_id.as_str()).unwrap();

        Bot::new(bot_id, low_id, high_id)
    })
}

fn parse_instruction(line: &str) -> Option<(usize, String)> {
    if !line.starts_with("value") {
        return None;
    }

    let re = Regex::new(r"value ([0-9]+) goes to (.* [0-9]+)").unwrap();

    re.captures(line).map(|cap| {
        let chip = cap
            .get(1)
            .map(|chip| chip.as_str().parse().unwrap())
            .unwrap();
        let target = cap.get(2).map(|target| target.as_str()).unwrap();

        (chip, target.to_string())
    })
}

fn update_bot(bots: &mut HashMap<String, Bot>, target: &str, chip: usize) {
    let (low_target, high_target, low_chip, high_chip, should_recurse) = {
        let target_bot = bots
            .entry(target.to_string())
            .or_insert_with(|| Bot::new(target, "", ""));

        target_bot.chips.push(chip);
        target_bot.chips.sort();

        // Check the condition and prepare data for recursion if necessary
        let should_recurse = target_bot.chips.len() == 2;

        let low_chip = target_bot.chips.first().cloned().unwrap_or_default();
        let high_chip = target_bot.chips.get(1).cloned().unwrap_or_default();

        if low_chip == 17 && high_chip == 61 {
            target_bot.is_responsible = true;
        }

        (
            target_bot.low_output.clone(),
            target_bot.high_output.clone(),
            low_chip,
            high_chip,
            should_recurse,
        )
    };

    // Now, the mutable borrow of `bots` has ended, so we can call recursively
    if should_recurse {
        update_bot(bots, &low_target, low_chip);
        update_bot(bots, &high_target, high_chip);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bots: HashMap<String, Bot> = input
        .lines()
        .filter_map(parse_bot_definition)
        .map(|b| (b.id.clone(), b))
        .collect();

    for (chip, bot) in input.lines().filter_map(parse_instruction) {
        update_bot(&mut bots, &bot, chip);
    }

    bots.iter().find(|(_, b)| b.is_responsible).map(|(bot, _)| {
        let (_, id) = bot.split_once(' ').unwrap();

        id.parse().unwrap()
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut bots: HashMap<String, Bot> = input
        .lines()
        .filter_map(parse_bot_definition)
        .map(|b| (b.id.clone(), b))
        .collect();

    for (chip, bot) in input.lines().filter_map(parse_instruction) {
        update_bot(&mut bots, &bot, chip);
    }

    let output0 = bots.get(&String::from("output 0")).unwrap().chips[0];
    let output1 = bots.get(&String::from("output 1")).unwrap().chips[0];
    let output2 = bots.get(&String::from("output 2")).unwrap().chips[0];

    Some(output0 * output1 * output2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(0, 0);
    }
}
