use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input
    .lines()
    .map(extract_groups)
    .map(|(first, second)| first.intersection(&second).count().try_into().unwrap())
    .filter(|num| num > &0)
    .map(|num: u32| u32::pow(2, num - 1))
    .sum()
}

fn extract_groups(input: &str) -> (HashSet<u32>, HashSet<u32>) {
    let mut first_group = HashSet::new();
    let mut second_group = HashSet::new();

    let parts: Vec<&str> = input.split("|").collect();
    if let Some(first_group_str) = parts.get(0) {
        first_group.extend(first_group_str.trim().split_whitespace().flat_map(|s| s.parse::<u32>()));
    }

    if let Some(second_group_str) = parts.get(1) {
        second_group.extend(second_group_str.trim().split_whitespace().flat_map(|s| s.parse::<u32>()));
    }

    (first_group, second_group)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 13);
    }


    #[test]
    fn test_extract_groups() {
        let result = extract_groups("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result, (HashSet::from([41, 48, 83, 86, 17]), HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])));
    }
}