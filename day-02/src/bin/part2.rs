use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let blue = "blue";
    let red = "red";
    let green = "green";

    let output = input.lines().map(|line| {
        let parts = line.split(":");
        let subsets_string = parts.last().expect("to be a string");
        let mut color_number_map = HashMap::from([
            (blue.to_string(), 0),
            (red.to_string(), 0),
            (green.to_string(), 0),
        ]);

        subsets_string.split(";").for_each(|subset| {
            let subset_color_number_map = get_colors_amount(subset);

            for (color, number) in &subset_color_number_map {
                if *number > *color_number_map.get(color).expect("to be a valid value") {
                    color_number_map.insert(color.to_string(), *number);
                }
            }
        });

        color_number_map.values().fold(1, |acc, elem| {
            acc * *elem
        })
    }).sum::<u32>();

    output
}

fn get_colors_amount(subset: &str) -> HashMap<String, u32> {
    let mut colors_amount = HashMap::new();

    let pattern = r"(\d+)\s+([a-zA-Z]+)";
    let re = Regex::new(pattern).expect("Failed to create regex");

    for capture in re.captures_iter(subset) {
        let number = capture[1].parse::<u32>().expect("Failed to parse number.");
        let captured_color = capture[2].to_string();

        colors_amount.insert(captured_color, number);
    }

    colors_amount
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}