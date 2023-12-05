use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}
fn find_and_store_symbols(input: &str) -> HashMap<(usize, usize), (Option<u32>, Option<u32>)> {
    let mut symbol_map = HashMap::new();

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, c) in line.char_indices() {
            if c == '*' {
                symbol_map.insert((row_idx, col_idx), (None, None));
            }
        }
    }

    symbol_map
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

fn find_numbers_adjacent_to_symbol(symbols: &mut HashMap<(usize, usize), (Option<u32>, Option<u32>)>, numbers: HashMap<(usize, usize), String>){
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
            if symbols.contains_key(&position) {
               let (curr_position, curr_numbers) = symbols.get_key_value(&position).expect("To exist a symbol in the key");
               match curr_numbers {
                (None, None) => symbols.insert(*curr_position, (Some(number.parse::<u32>().expect("to be a valid number")), None)),
                (Some(x), None) => symbols.insert(*curr_position, (Some(*x), Some(number.parse::<u32>().expect("to be a valid number")))),
                _ => None
               };
            }
        }
    }
}

fn part2(input: &str) -> u32 {
    // Idea 1 for part:
    // get all the positions of Symbols in a hash_Map, like (row, col) -> (None, None),
    // where the value are the numbers adjacent to them, initially None and None, possibly Some(n1), Some(n2)
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

    let mut symbol_map = find_and_store_symbols(input);
    let number_map = find_and_store_numbers(input);

    find_numbers_adjacent_to_symbol(&mut symbol_map, number_map);

    let mut result = 0;

    for (_, numbers) in symbol_map {
        match numbers {
            (Some(x), Some(y)) => result = result + x * y,
            _ => ()
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("
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
        assert_eq!(result, 467835);
    }
}