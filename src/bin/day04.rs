use glam::IVec2;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day04.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let result = part_2(&parsed_input);
    println!("part_2: {result}");
}

fn parse(input: &str) -> HashMap<IVec2, char> {
    let mut map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            map.insert(IVec2::new(x as i32, y as i32), c);
        }
    }
    map
}

fn _print_grid(grid: &HashMap<IVec2, char>) {
    for x in 0..10 {
        for y in 0..10 {
            if let Some(c) = grid.get(&IVec2::new(x, y)) {
                print!("{c}");
            }
        }
        println!();
    }
}

fn part_1(input: &HashMap<IVec2, char>) -> i32 {
    //print_grid(input);
    let max_x = input.keys().map(|pos| pos.x).max().unwrap();
    let max_y = input.keys().map(|pos| pos.y).max().unwrap();
    let mut total = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            let curr_pos = IVec2::new(x, y);
            let Some(c) = input.get(&curr_pos) else {
                continue;
            };
            if *c != 'X' {
                continue;
            }
            for dir in [
                IVec2::new(1, 0),
                IVec2::new(1, 1),
                IVec2::new(1, -1),
                IVec2::new(0, 1),
                IVec2::new(-1, 0),
                IVec2::new(-1, -1),
                IVec2::new(-1, 1),
                IVec2::new(0, -1),
            ] {
                let mut next_pos = curr_pos;
                for next_c in ['M', 'A', 'S'] {
                    next_pos += dir;
                    let Some(next) = input.get(&next_pos) else {
                        break;
                    };
                    if *next == next_c {
                        if next_c == 'S' {
                            total += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    total
}

fn part_2(input: &HashMap<IVec2, char>) -> i32 {
    let mut total = 0;
    let max_x = input.keys().map(|pos| pos.x).max().unwrap();
    let max_y = input.keys().map(|pos| pos.y).max().unwrap();
    for x in 0..=max_x {
        for y in 0..=max_y {
            let curr_pos = IVec2::new(x, y);
            let Some(c) = input.get(&curr_pos) else {
                continue;
            };
            if *c != 'A' {
                continue;
            }
            let mut m_corners = vec![];
            let mut s_corners = vec![];
            for dir in [
                IVec2::new(1, 1),
                IVec2::new(1, -1),
                IVec2::new(-1, -1),
                IVec2::new(-1, 1),
            ] {
                let corner = curr_pos + dir;
                if let Some(corner_char) = input.get(&corner) {
                    match corner_char {
                        'M' => m_corners.push(corner),
                        'S' => s_corners.push(corner),
                        _ => {
                            // todo early out
                        }
                    }
                }
            }
            if m_corners.len() != 2 || s_corners.len() != 2 {
                continue;
            }
            if m_corners[0].x == m_corners[1].x || m_corners[0].y == m_corners[1].y {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 18);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 9);
    }
}
