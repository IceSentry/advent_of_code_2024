type Data = Vec<i32>;

fn main() {
    let input = std::fs::read_to_string("inputs/day02.txt").unwrap();
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
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &[i32]) -> bool {
    let direction = (report[0] - report[1]).signum();
    for level in report.windows(2) {
        let &[a, b] = level else { continue };
        if (a - b).signum() != direction {
            return false;
        }
        let diff = (a - b).abs();
        if diff > 3 || diff == 0 {
            return false;
        }
    }
    true
}

fn part_1(reports: &[Data]) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        if is_report_safe(report) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn part_2(reports: &[Data]) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        if is_report_safe(report) {
            safe_reports += 1;
            continue;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if is_report_safe(&new_report) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 4);
    }
}
