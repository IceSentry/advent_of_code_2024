use std::str::Chars;

fn main() {
    let input = std::fs::read_to_string("inputs/day03.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(parsed_input);
    println!("part_1: {result}");
    let result = part_2(parsed_input);
    println!("part_2: {result}");
}

fn parse(input: &str) -> &str {
    input
}

// tries to match a str, if the string matches advance the iterator by its len. Otherwise the
// iterator will not be advanced
fn peek_match(chars: &mut Chars, search: &str) -> bool {
    let mut peekable = chars.clone().peekable();
    for c in search.chars() {
        let Some(peek) = peekable.peek() else {
            return false;
        };
        if *peek == c {
            peekable.next();
        } else {
            return false;
        }
    }
    for _ in 0..search.len() {
        chars.next();
    }
    true
}

fn get_number(chars: &mut Chars, end_char: char) -> Option<i32> {
    let mut number = 0;
    for c in chars.by_ref() {
        if c.is_ascii_digit() {
            number *= 10;
            number += c.to_digit(10).unwrap();
        } else if c == end_char {
            break;
        } else {
            return None;
        }
    }
    Some(number as i32)
}

fn part_1(input: &str) -> i32 {
    let mut total = 0;
    let mut chars = input.chars();
    'next: loop {
        if peek_match(&mut chars, "mul(") {
            //println!("mul( found");
            let Some(left_number) = get_number(&mut chars, ',') else {
                continue 'next;
            };
            let Some(right_number) = get_number(&mut chars, ')') else {
                continue 'next;
            };
            //println!("{left_number} x {right_number}");
            total += left_number * right_number;
        } else if chars.next().is_none() {
            break;
        }
    }
    total
}

fn part_2(input: &str) -> i32 {
    let mut total = 0;
    let mut chars = input.chars();
    let mut enabled = true;
    'next: loop {
        if peek_match(&mut chars, "mul(") {
            let Some(left_number) = get_number(&mut chars, ',') else {
                continue 'next;
            };
            let Some(right_number) = get_number(&mut chars, ')') else {
                continue 'next;
            };
            if enabled {
                total += left_number * right_number;
            }
        } else if peek_match(&mut chars, "do()") {
            enabled = true;
        } else if peek_match(&mut chars, "don't()") {
            enabled = false;
        } else if chars.next().is_none() {
            break;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn part_1() {
        let result = super::part_1(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(result, 161);
    }

    #[test]
    pub fn part_2() {
        let result = super::part_2(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, 48);
    }
}
