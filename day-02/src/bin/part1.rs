use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let blue = "blue";
    let red = "red";
    let green = "green";

    let max_blue_blocks = 14;
    let max_red_blocks = 12;
    let max_green_blocks = 13;

    let output = input.lines().map(|line| {
        let mut parts = line.split(":");
        let game_id_string = parts.next().expect("to be a string");
        let subsets_string = parts.next().expect("to be a string");

        let valid_game = subsets_string.split(";").map(|subset| {
            let color_number_map = get_colors_amount(subset);
            
            let blue_blocks = if color_number_map.contains_key(blue) {
                *color_number_map.get(blue).expect("to contain a value for the key")
            } else {
                0
            };

            let red_blocks = if color_number_map.contains_key(red) {
                *color_number_map.get(red).expect("to contain a value for the key")
            } else {
                0
            };

            let green_blocks = if color_number_map.contains_key(green) {
                *color_number_map.get(green).expect("to contain a value for the key")
            } else {
                0
            };

            blue_blocks <= max_blue_blocks &&
            red_blocks <= max_red_blocks &&
            green_blocks <= max_green_blocks
        }).all(|item| {
            item
        });

        if valid_game {
            let id = game_id_string.split(" ").last().expect("to be a string");
    
            return id.parse::<u32>().expect("should be a valid number")
        }

        0
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
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}