use std::collections::HashMap;

type Data = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let result = part_2(&parsed_input);
    println!("part_2: {result}");
}

fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("   ").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn get_lists(input: &[Data]) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for (left, right) in input {
        left_list.push(*left);
        right_list.push(*right);
    }
    left_list.sort();
    right_list.sort();
    (left_list, right_list)
}

fn part_1(input: &[Data]) -> i32 {
    let (left_list, right_list) = get_lists(input);
    let mut sum = 0;
    for (left, right) in left_list.iter().zip(right_list) {
        let distance = (left - right).abs();
        sum += distance;
    }
    sum
}

fn part_2(input: &[Data]) -> i32 {
    let (left_list, right_list) = get_lists(input);
    let mut map = HashMap::new();
    for right in right_list {
        let entry = map.entry(right).or_insert(0);
        *entry += 1;
    }
    let mut total = 0;
    for left in left_list {
        total += left * map.get(&left).unwrap_or(&0);
    }
    total
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 31);
    }
}
