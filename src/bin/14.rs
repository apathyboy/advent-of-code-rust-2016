use itertools::Itertools;

advent_of_code::solution!(14);

fn find_first_triple(input: &str) -> Option<char> {
    input
        .chars()
        .tuple_windows()
        .filter(|(a, b, c)| *a == *b && *a == *c)
        .map(|(a, _, _)| a)
        .next()
}

fn make_key_stretching_hash(input: &str) -> String {
    let mut hash = advent_of_code::make_hash(input);

    for _ in 0..2016 {
        hash = advent_of_code::make_hash(&hash);
    }

    hash
}

pub fn part_one(input: &str) -> Option<u32> {
    let salt = input.trim();

    // make this a hashmap of key (String, u32) and the value as a bool flag for if it is a key
    let mut triples: Vec<(String, u32, bool)> = Vec::new();

    for i in 0.. {
        //if i > 1000 {
        //    panic!("stop");
        //}
        let secret = advent_of_code::make_secret(salt, i);
        let hash = advent_of_code::make_hash(&secret);

        // check for key
        triples.retain(|&(_, idx, is_key)| is_key || idx > if i >= 1000 { i - 1000 } else { 0 });

        //println!("{:?}", &triples);

        for (triple, _, is_key) in triples.iter_mut().filter(|(_, _, is_key)| !is_key) {
            let triple: &str = triple;
            let check = triple.repeat(5);

            if hash.contains(&check) {
                *is_key = true;
            }
        }

        let check_for_key = triples.iter().filter(|(_, _, is_key)| *is_key).nth(63);

        if let Some((_, idx, _)) = check_for_key {
            return Some(*idx);
        }

        // check for triple

        let triple = find_first_triple(&hash);

        if let Some(triple) = triple {
            //println!("Triple found: {} at {}", triple, i);
            //if !triples.contains_key(&triple.to_string()) {
            triples.push((triple.to_string(), i, false));
            //}
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let salt = input.trim();

    // make this a hashmap of key (String, u32) and the value as a bool flag for if it is a key
    let mut triples: Vec<(String, u32, bool)> = Vec::new();

    for i in 0.. {
        //if i > 1000 {
        //    panic!("stop");
        //}
        let secret = advent_of_code::make_secret(salt, i);
        let hash = make_key_stretching_hash(&secret);

        // check for key
        triples.retain(|&(_, idx, is_key)| is_key || idx > if i >= 1000 { i - 1000 } else { 0 });

        //println!("{:?}", &triples);

        for (triple, _, is_key) in triples.iter_mut().filter(|(_, _, is_key)| !is_key) {
            let triple: &str = triple;
            let check = triple.repeat(5);

            if hash.contains(&check) {
                *is_key = true;
            }
        }

        let check_for_key = triples.iter().filter(|(_, _, is_key)| *is_key).nth(63);

        if let Some((_, idx, _)) = check_for_key {
            return Some(*idx);
        }

        // check for triple

        let triple = find_first_triple(&hash);

        if let Some(triple) = triple {
            //println!("Triple found: {} at {}", triple, i);
            //if !triples.contains_key(&triple.to_string()) {
            triples.push((triple.to_string(), i, false));
            //}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_stretching_hash() {
        let result = make_key_stretching_hash("abc0");
        assert_eq!(result, "a107ff634856bb300138cac6568c0f24".to_string());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22728));
    }
}
