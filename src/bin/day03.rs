use core::panic;
use std::str::Chars;

fn main() {
    let input = std::fs::read_to_string("inputs/day03.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let result = part_2(&parsed_input);
    println!("part_2: {result}");
}

fn parse(input: &str) -> &str {
    input
}

fn match_str(chars: &mut Chars, str: &str) -> bool {
    for c in str.chars() {
        if chars.next().unwrap() != c {
            return false;
        }
    }
    true
}

fn part_1(input: &str) -> i32 {
    let mut total = 0;
    let mut chars = input.chars();
    'next: while let Some(c) = chars.next() {
        if c == 'm' && match_str(&mut chars, "ul(") {
            //println!("mul( found");
            let mut left_number = 0;
            let mut right_number = 0;
            for c in chars.by_ref() {
                if c.is_ascii_digit() {
                    left_number *= 10;
                    left_number += c.to_digit(10).unwrap();
                    continue;
                } else if c == ',' {
                    break;
                }
                continue 'next;
            }
            for c in chars.by_ref() {
                if c.is_ascii_digit() {
                    right_number *= 10;
                    right_number += c.to_digit(10).unwrap();
                    continue;
                } else if c == ')' {
                    break;
                }
                continue 'next;
            }
            //println!("{left_number} x {right_number}");
            total += left_number * right_number;
        }
    }
    total as i32
}

fn part_2(input: &str) -> i32 {
    let mut total = 0;
    let mut chars = input.chars();
    let mut enabled = true;
    'next: while let Some(c) = chars.next() {
        if c == 'm' && match_str(&mut chars, "ul(") {
            //println!("mul( found");
            let mut left_number = 0;
            let mut right_number = 0;
            for c in chars.by_ref() {
                if c.is_ascii_digit() {
                    left_number *= 10;
                    left_number += c.to_digit(10).unwrap();
                    continue;
                } else if c == ',' {
                    break;
                }
                continue 'next;
            }
            for c in chars.by_ref() {
                if c.is_ascii_digit() {
                    right_number *= 10;
                    right_number += c.to_digit(10).unwrap();
                    continue;
                } else if c == ')' {
                    break;
                }
                continue 'next;
            }
            //println!("{left_number} x {right_number}");
            if enabled {
                total += left_number * right_number;
            }
        } else if c == 'd' {
            if let Some('o') = chars.next() {
                if let Some(c) = chars.next() {
                    if c == '(' {
                        if let Some(')') = chars.next() {
                            // enabled
                            enabled = true;
                        }
                    } else if c == 'n' && match_str(&mut chars, "'t()") {
                        // disabled
                        enabled = false;
                    }
                }
            }
        }
    }
    total as i32
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
