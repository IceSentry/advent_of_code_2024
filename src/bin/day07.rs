use fxhash::FxHashMap as HashMap;
use std::fmt::Debug;

type Data = (u64, Vec<u64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn main() {
    //generate_permutations(&['x', '+'], 3, &mut HashMap::default());
    //return;
    let input = std::fs::read_to_string("inputs/day07.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let start = std::time::Instant::now();
    let result = part_2(&parsed_input);
    let elapsed = start.elapsed().as_millis();
    println!("part_2: {result} {elapsed}ms");
}

fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            let (expected_result, numbers) = l.split_once(':').unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (expected_result.parse().unwrap(), numbers)
        })
        .collect()
}

fn generate_permutations<T: Clone + Copy + Debug>(
    items: &[T],
    len: usize,
    cache: &mut HashMap<usize, Vec<Vec<T>>>,
) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    if len == 1 {
        let mut current = Vec::new();
        generate(items, len, &mut current, &mut result);
        return result;
    } else if let Some(cached) = cache.get(&len) {
        return cached.clone();
    } else {
        let generated_permutations = generate_permutations(items, len - 1, cache);
        for item in generated_permutations {
            let mut current = item.clone();
            generate(items, len, &mut current, &mut result);
        }
        cache.insert(len, result.clone());
    }
    result
}

fn generate<T: Clone + Copy>(
    items: &[T],
    len: usize,
    current: &mut Vec<T>,
    result: &mut Vec<Vec<T>>,
) {
    if current.len() == len {
        result.push(current.clone());
    } else {
        for &c in items {
            current.push(c);
            generate(items, len, current, result);
            current.pop();
        }
    }
}

fn count_digits(n: u64, base: u64) -> u64 {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}

fn test_permutations(ops_permutations: &[Vec<Op>], numbers: &[u64], expected_result: u64) -> bool {
    'next: for ops in ops_permutations {
        let mut ops = ops.iter();
        let mut numbers = numbers.iter();
        let Some(first) = numbers.next() else {
            unreachable!()
        };
        let mut result = *first;
        while let (Some(op), Some(num)) = (ops.next(), numbers.next()) {
            match op {
                Op::Add => result += num,
                Op::Mul => result *= num,
                Op::Concat => result = result * (10 * count_digits(*num, 10)) + num,
            }
            if result > expected_result {
                continue 'next;
            }
        }
        //println!("{result}");
        if result == expected_result {
            return true;
        }
    }
    false
}

fn part_1(input: &[Data]) -> u64 {
    let mut total = 0;
    let mut permutations = HashMap::default();
    for (expected_result, numbers) in input {
        let generated_permutations =
            generate_permutations(&[Op::Add, Op::Mul], numbers.len() - 1, &mut permutations);
        if test_permutations(&generated_permutations, numbers, *expected_result) {
            total += expected_result;
        }
    }
    total
}

fn part_2(input: &[Data]) -> u64 {
    let mut total = 0;
    let mut permutations = HashMap::default();
    for (expected_result, numbers) in input {
        let generated_permutations = generate_permutations(
            &[Op::Add, Op::Mul, Op::Concat],
            numbers.len() - 1,
            &mut permutations,
        );
        if test_permutations(&generated_permutations, numbers, *expected_result) {
            total += expected_result;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 3749);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 11387);
    }
}
