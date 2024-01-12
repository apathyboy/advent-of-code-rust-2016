use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Bot {
    low_output: String,
    high_output: String,
    chips: Vec<usize>,
}

impl Bot {
    fn new(low_output: &str, high_output: &str, chips: Vec<usize>) -> Self {
        Self {
            low_output: String::from(low_output),
            high_output: String::from(high_output),
            chips,
        }
    }
}

lazy_static! {
    static ref BOT_REGEX: Regex =
        Regex::new(r"(bot [0-9]+) gives low to (.* [0-9]+) and high to (.* [0-9]+)").unwrap();
    static ref VALUE_REGEX: Regex = Regex::new(r"value ([0-9]+) goes to (.* [0-9]+)").unwrap();
}

fn parse_bot_definition(line: &str) -> Option<(String, Bot)> {
    if !line.starts_with("bot") {
        return None;
    }

    BOT_REGEX.captures(line).map(|cap| {
        let bot_id = cap.get(1).map(|bot_id| bot_id.as_str()).unwrap();
        let low_id = cap.get(2).map(|low_id| low_id.as_str()).unwrap();
        let high_id = cap.get(3).map(|high_id| high_id.as_str()).unwrap();

        (bot_id.to_string(), Bot::new(low_id, high_id, Vec::new()))
    })
}

fn parse_instruction(line: &str) -> Option<(usize, String)> {
    if !line.starts_with("value") {
        return None;
    }

    VALUE_REGEX.captures(line).map(|cap| {
        let chip = cap
            .get(1)
            .map(|chip| chip.as_str().parse().unwrap())
            .unwrap();
        let target = cap.get(2).map(|target| target.as_str()).unwrap();

        (chip, target.to_string())
    })
}

fn update_bot(bots: &mut HashMap<String, Bot>, target: &str, chip: usize) {
    if let Some(target_bot) = bots.get_mut(target) {
        target_bot.chips.push(chip);
        target_bot.chips.sort_unstable(); // `sort_unstable` is often faster and suitable here

        if target_bot.chips.len() == 2 {
            let [min_chip, max_chip] = [target_bot.chips[0], target_bot.chips[1]];
            let low_output = target_bot.low_output.clone();
            let high_output = target_bot.high_output.clone();

            let _ = target_bot;

            update_bot(bots, &low_output, min_chip);
            update_bot(bots, &high_output, max_chip);
        }
    } else {
        // Handle the case where the target bot does not exist
        bots.insert(target.to_string(), Bot::new("", "", vec![chip]));
    }
}

fn run_bots(input: &str) -> HashMap<String, Bot> {
    let mut bots: HashMap<String, Bot> = HashMap::new();

    // Process bot definitions
    input
        .lines()
        .filter_map(parse_bot_definition)
        .for_each(|(id, bot)| {
            bots.insert(id.to_owned(), bot); // Insert bot into HashMap
        });

    // Process instructions
    for (chip, target) in input.lines().filter_map(parse_instruction) {
        update_bot(&mut bots, &target, chip);
    }

    bots
}

pub fn part_one(input: &str) -> Option<u32> {
    run_bots(input).iter().find_map(|(bot, b)| {
        if b.chips.len() == 2 && b.chips[0] == 17 && b.chips[1] == 61 {
            bot.split_once(' ').and_then(|(_, id)| id.parse().ok())
        } else {
            None
        }
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    run_bots(input)
        .iter()
        .filter(|(id, _)| ["output 0", "output 1", "output 2"].contains(&id.as_str()))
        .map(|(_, bot)| bot.chips[0])
        .try_fold(1, |acc, x| Some(acc * x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_bots() {
        let bots = run_bots(&advent_of_code::template::read_file("examples", DAY));
        let output0 = bots.get("output 0").and_then(|b| b.chips.first());
        let output2 = bots.get("output 2").and_then(|b| b.chips.first());

        assert_eq!(output0, Some(&5));
        assert_eq!(output2, Some(&3));
    }
}
