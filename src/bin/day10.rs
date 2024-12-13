use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use glam::IVec2;

fn main() {
    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
    let parsed_input = parse(tests::INPUT);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let result = part_2(&parsed_input);
    println!("part_2: {result}");
}

fn parse(input: &str) -> HashMap<IVec2, i32> {
    let mut map = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            map.insert(
                IVec2::new(x as i32, y as i32),
                c.to_digit(10).unwrap() as i32,
            );
        }
    }
    map
}

fn part_1(map: &HashMap<IVec2, i32>) -> i32 {
    let mut start_pos = Vec::new();
    for (pos, val) in map {
        if *val == 0 {
            start_pos.push(pos);
        }
    }
    let mut total = 0;
    for start in start_pos {
        // todo check neighbors
        let val = follow_path(map, start);
        total += val;
        println!("{val}");
    }
    total
}

fn follow_path(map: &HashMap<IVec2, i32>, start: &IVec2) -> i32 {
    let Some(start_val) = map.get(start) else {
        return 0;
    };
    if *start_val == 9 {
        return 1;
    }
    let mut total = 0;
    for pos in [
        IVec2::new(1, 0),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(0, -1),
    ] {
        if let Some(neighbor) = map.get(&(start + pos)) {
            println!("found {start},{start_val} -> {},{neighbor}", start + pos);
            if *neighbor == start_val + 1 {
                total += follow_path(map, &(start + pos));
            }
        }
    }
    total
}

fn part_2(_input: &HashMap<IVec2, i32>) -> i32 {
    0
}

//#[cfg(test)]
pub mod tests {
    pub const INPUT: &str = "
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 36);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 0);
    }
}
