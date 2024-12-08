fn main() {
    let input = std::fs::read_to_string("inputs/day05.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let result = part_2(&parsed_input);
    println!("part_2: {result}");
}

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let ordering_rules = ordering_rules
        .lines()
        .map(|rule| {
            let (x, y) = rule.split_once('|').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<_>>();
    let pages_to_produce = updates
        .lines()
        .map(|l| l.split(',').map(|page| page.parse().unwrap()).collect())
        .collect::<Vec<_>>();
    (ordering_rules, pages_to_produce)
}

fn is_update_valid(update: &[i32], ordering_rules: &[(i32, i32)]) -> bool {
    for (i, page) in update.iter().enumerate() {
        for (x, y) in ordering_rules {
            if x == page && update[0..i].contains(y) {
                return false;
            }
        }
    }
    true
}

fn part_1((ordering_rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut valid_updates = vec![];
    for update in updates {
        if is_update_valid(update, ordering_rules) {
            valid_updates.push(update);
        }
    }
    let mut result = 0;
    for update in valid_updates {
        result += update[update.len() / 2];
    }
    result
}

fn fix_update(update: &[i32], ordering_rules: &[(i32, i32)]) -> Vec<i32> {
    let mut fixed_update = update.to_vec();
    loop {
        let update = fixed_update.clone();
        let mut is_valid = true;
        'iter: for (i, page) in update.iter().enumerate() {
            for (x, y) in ordering_rules {
                if x != page {
                    continue;
                }
                for (ii, ppage) in update[0..i].iter().enumerate() {
                    if ppage != y {
                        continue;
                    }
                    // move the invalid page after the current page
                    fixed_update.remove(ii);
                    fixed_update.insert(i, *ppage);
                    is_valid = false;
                    break 'iter;
                }
            }
        }
        if is_valid {
            break;
        }
    }
    fixed_update
}

fn part_2((ordering_rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut invalid_updates = vec![];
    for update in updates {
        if !is_update_valid(update, ordering_rules) {
            invalid_updates.push(update);
        }
    }
    let mut fixed_updates = vec![];
    for invalid_update in invalid_updates {
        fixed_updates.push(fix_update(invalid_update, ordering_rules));
    }
    let mut result = 0;
    for update in fixed_updates {
        result += update[update.len() / 2];
    }
    result
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 143);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 123);
    }
}
