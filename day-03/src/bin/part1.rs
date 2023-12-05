use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_and_store_symbols(input: &str) -> HashSet<(usize, usize)> {
    let mut symbol_set = HashSet::new();

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, c) in line.char_indices() {
            if !c.is_digit(10) && c != '.' && c != ' ' {
                symbol_set.insert((row_idx, col_idx));
            }
        }
    }

    symbol_set
}

fn find_and_store_numbers(input: &str) -> HashMap<(usize, usize), String> {
    let mut number_map = HashMap::new();

    let mut current_number = String::new();
    let mut current_position = None;

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, c) in line.char_indices() {
            if c.is_digit(10) {
                current_number.push(c);
                current_position.get_or_insert((row_idx, col_idx));
            } else if !current_number.is_empty() {
                number_map.insert(current_position.unwrap(), current_number.clone());
                current_number.clear();
                current_position = None;
            }
        }
    }

    number_map
}

fn find_numbers_adjacent_to_symbol(symbols: HashSet<(usize, usize)>, numbers: HashMap<(usize, usize), String>) -> Vec<u32> {
    let mut adjacent_numbers = vec![];
    for (position, number) in numbers {
        let number_length = number.len();
        let (row, initial_col) = position;
        let prev_col = if initial_col == 0 { 0 } else { initial_col - 1};
        let post_col = initial_col + number_length;
        let mut positions_to_check = vec![];
        if initial_col > 0 {
            positions_to_check.push((row, initial_col - 1));
        }
        positions_to_check.push((row, post_col));
        if row > 0 {
            for col in prev_col..=post_col {
                positions_to_check.push((row - 1, col));
            }
        }
        for col in prev_col..=post_col {
            positions_to_check.push((row + 1, col));
        }

        for position in positions_to_check {
            if symbols.contains(&position) {
                adjacent_numbers.push(number.parse::<u32>().expect("must be a number"));
                break;
            }
        }
    }

    adjacent_numbers
}

fn part1(input: &str) -> u32 {
    // Idea 1:
    // get all the positions of Symbols in a hash_Set, like (row, col)
    // find all the numbers with his initial positions, and save them to a map,
    // such as the position is the key, the number is the value, like (row, col) -> number
    // check if there is any symbol around the number:
    // *****
    // *374*
    // *****
    // (row, start_col - 1) 
    // (row, end_col + 1)
    // (row - 1, [(start_col - 1)..(end_col + 1)])
    // (row + 1, [(start_col - 1)..(end_col + 1)])

    let symbol_set = find_and_store_symbols(input);
    let number_map = find_and_store_numbers(input);

    let result = find_numbers_adjacent_to_symbol(symbol_set, number_map).iter().sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..");
        assert_eq!(result, 4361);
    }
}