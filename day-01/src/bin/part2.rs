fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum: u32 = 0;
    
    for line in lines {
        let calibration_value = calibration_value(line);
        sum = sum + calibration_value;
    }
    sum
}

fn calibration_value(line: &str) -> u32 {
    let numeric_words = [
        "one", "1",
        "two", "2",
        "three", "3",
        "four", "4",
        "five", "5",
        "six", "6",
        "seven", "7",
        "eight", "8",
        "nine", "9",
    ];

    let first_numeric = find_first_string(line, &numeric_words);
    let last_numeric = find_last_string(line, &numeric_words);

    if let (Some(first), Some(last)) = (first_numeric, last_numeric) {
        let first_digit_option = match_number_word(&first);
        let last_digit_option = match_number_word(&last);

        if let (Some(first_digit), Some(last_digit)) = (first_digit_option, last_digit_option) {
            return first_digit * 10 + last_digit;
        }
    }

    0
}

fn find_first_string(input: &str, words: &[&str]) -> Option<String> {
    for i in 0..input.len() {
        for word in words {
            if input[i..].starts_with(word) {
                return Some(word.to_string());
            }
        }
    }

    None
}

fn find_last_string(input: &str, words: &[&str]) -> Option<String> {
    for i in (0..input.len() + 1).rev() {
        for word in words {
            if input[..i].ends_with(word) {
                return Some(word.to_string());
            }
        }
    }

    None
}

fn match_number_word(word: &str) -> Option<u32> {
    match word.to_lowercase().as_str() {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, 281);
    }

    #[test]
    fn test_match() {
        let result = match_number_word("one");
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_find_first_string() {
        let input = "bcone2threexyz";
        let words = ["one"];
        let result = find_first_string(input, &words);
        assert_eq!(result, Some("one".to_string()));
    }

    #[test]
    fn test_find_last_string() {
        let input = "bcone2three";
        let words = ["one", "three"];
        let result = find_last_string(input, &words);
        assert_eq!(result, Some("three".to_string()));
    }
}