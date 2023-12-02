fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum: u32 = 0;
    for line in lines {
        let first_numeric_char = line.chars().find(|c| c.is_numeric());
        let last_numeric_char = line.chars().rev().find(|c| c.is_numeric());

        if let (Some(first), Some(last)) = (first_numeric_char, last_numeric_char) {
            let first_numeric = first.to_digit(10).unwrap();
            let last_numeric = last.to_digit(10).unwrap();

            let resulting_number = first_numeric * 10 + last_numeric;
            sum = sum + resulting_number;
        }  
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142);
    }
}