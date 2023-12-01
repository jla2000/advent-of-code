pub fn day1_part1() -> u32 {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    input.lines().fold(0, |result, line| {
        let iter = line.chars().filter_map(|c| c.to_digit(10));
        result + iter.clone().next().unwrap() * 10 + iter.last().unwrap()
    })
}

pub fn day1_part2() -> u32 {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    input.lines().fold(0, |result, line| {
        let chars = line.chars().collect::<Vec<char>>();
        let iter = (0..chars.len()).filter_map(|idx| match &chars[idx..chars.len()] {
            ['0', ..] | ['z', 'e', 'r', 'o', ..] => Some(0),
            ['1', ..] | ['o', 'n', 'e', ..] => Some(1),
            ['2', ..] | ['t', 'w', 'o', ..] => Some(2),
            ['3', ..] | ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
            ['4', ..] | ['f', 'o', 'u', 'r', ..] => Some(4),
            ['5', ..] | ['f', 'i', 'v', 'e', ..] => Some(5),
            ['6', ..] | ['s', 'i', 'x', ..] => Some(6),
            ['7', ..] | ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
            ['8', ..] | ['e', 'i', 'g', 'h', 't', ..] => Some(8),
            ['9', ..] | ['n', 'i', 'n', 'e', ..] => Some(9),
            _ => None,
        });
        result + iter.clone().next().unwrap() * 10 + iter.last().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_test() {
        println!("Result of challenge: {}", day1_part1());
    }

    #[test]
    fn day1_part2_test() {
        println!("Result of challenge: {}", day1_part2());
    }
}
