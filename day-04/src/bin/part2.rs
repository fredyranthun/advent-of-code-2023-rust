use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {

    let total_cards = input.lines().count();
    let mut number_of_cards = vec![1; total_cards];

    let mut pos = 0;
    for line in input.lines().into_iter() {
        let (first_group, second_group) = extract_groups(line);
        let amount_card = number_of_cards[pos];
        let mut in_first_group = 0;

        for num in second_group.iter() {
            if first_group.contains(num) {
                in_first_group += 1;
            }
        }
        if in_first_group != 0 {
            for i in (pos + 1)..(pos + 1 + in_first_group) {       
                if i >= number_of_cards.len() {
                    break;
                }
                number_of_cards[i] += amount_card;
            }
        }
        pos += 1;
    }

    number_of_cards.iter().sum()
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

fn in_first_group(groups: (HashSet<u32>, HashSet<u32>)) -> u32 {
    groups.1.iter()
    .map(|num| {
        if groups.0.contains(num) { 1 } else { 0 }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 30);
    }


    #[test]
    fn test_extract_groups() {
        let result = extract_groups("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result, (HashSet::from([41, 48, 83, 86, 17]), HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])));
    }
}