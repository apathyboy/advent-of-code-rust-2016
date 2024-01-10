advent_of_code::solution!(9);

fn decompressed_len_v1(compressed: &str) -> usize {
    let mut len = 0;
    let mut cur_idx = 0;

    loop {
        let sequence_len = compressed
            .chars()
            .skip(cur_idx)
            .take_while(|c| *c != '(')
            .count();

        len += sequence_len;
        cur_idx += sequence_len;

        if cur_idx >= compressed.len() {
            break;
        }

        let marker = compressed
            .chars()
            .skip(cur_idx + 1)
            .take_while(|c| *c != ')')
            .collect::<String>();
        let (group_len, repeat) = marker.split_once('x').unwrap();
        let group_len = group_len.parse::<usize>().unwrap();
        let repeat = repeat.parse::<usize>().unwrap();

        cur_idx += marker.len() + 2 + group_len;
        len += group_len * repeat;

        if cur_idx >= compressed.len() {
            break;
        }
    }

    len
}

fn decompressed_len_v2(compressed: &str) -> usize {
    let mut len = 0;
    let mut cur_idx = 0;

    loop {
        let sequence_len = compressed
            .chars()
            .skip(cur_idx)
            .take_while(|c| *c != '(')
            .count();

        len += sequence_len;
        cur_idx += sequence_len;

        if cur_idx >= compressed.len() {
            break;
        }

        let marker = compressed
            .chars()
            .skip(cur_idx + 1)
            .take_while(|c| *c != ')')
            .collect::<String>();
        let (group_len, repeat) = marker.split_once('x').unwrap();
        let group_len = group_len.parse::<usize>().unwrap();
        let repeat = repeat.parse::<usize>().unwrap();

        cur_idx += marker.len() + 2;
        len += decompressed_len_v2(&compressed[cur_idx..cur_idx + group_len]) * repeat;
        cur_idx += group_len;

        if cur_idx >= compressed.len() {
            break;
        }
    }

    len
}

pub fn part_one(input: &str) -> Option<usize> {
    //Some(decompress(input.trim()).len())
    Some(decompressed_len_v1(input.trim()))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(decompressed_len_v2(input.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_compression_len() {
        let result = decompressed_len_v1("ADVENT");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_decompression_len_v1() {
        let result = decompressed_len_v1("A(1x5)BC");
        assert_eq!(result, 7);

        let result = decompressed_len_v1("(3x3)XYZ");
        assert_eq!(result, 9);

        let result = decompressed_len_v1("A(2x2)BCD(2x2)EFG");
        assert_eq!(result, 11);

        let result = decompressed_len_v1("(6x1)(1x3)A");
        assert_eq!(result, 6);

        let result = decompressed_len_v1("X(8x2)(3x3)ABCY");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_decompression_len_v2() {
        let result = decompressed_len_v2("(3x3)XYZ");
        assert_eq!(result, 9);

        let result = decompressed_len_v2("X(8x2)(3x3)ABCY");
        assert_eq!(result, 20);

        let result = decompressed_len_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A");
        assert_eq!(result, 241920);

        let result =
            decompressed_len_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");
        assert_eq!(result, 445);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(241920));
    }
}
