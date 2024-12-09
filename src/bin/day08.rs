use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/day08.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let result = part_2(&parsed_input);
    println!("part_2: {result}");
}

struct Bounds {
    min: IVec2,
    max: IVec2,
}

fn parse(input: &str) -> (Bounds, HashMap<char, Vec<IVec2>>) {
    let mut map = HashMap::default();
    let bounds = Bounds {
        min: IVec2::ZERO,
        max: IVec2::new(
            input.lines().next().unwrap().chars().count() as i32,
            input.lines().count() as i32,
        ),
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            map.entry(c)
                .or_insert(Vec::new())
                .push(IVec2::new(x as i32, y as i32));
        }
    }
    (bounds, map)
}

fn check_bounds(pos: IVec2, bounds: &Bounds) -> bool {
    if pos.x < bounds.min.x || pos.x >= bounds.max.x {
        return false;
    }
    if pos.y < bounds.min.y || pos.y >= bounds.max.y {
        return false;
    }
    true
}

fn _print_map(bounds: &Bounds, map: &HashMap<char, Vec<IVec2>>, antinode_map: &HashSet<IVec2>) {
    for y in 0..bounds.max.y {
        for x in 0..bounds.max.x {
            let mut found = false;
            for (freq, antennas) in map {
                if antennas.contains(&IVec2::new(x, y)) {
                    print!("{freq}");
                    found = true;
                    break;
                }
            }
            if !found {
                if antinode_map.contains(&IVec2::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn part_1((bounds, map): &(Bounds, HashMap<char, Vec<IVec2>>)) -> i32 {
    let mut antinode_map = HashSet::default();
    for antennas in map.values() {
        for x in antennas.iter().combinations(2) {
            let a = x[0];
            let b = x[1];
            let ab = a - b;
            let new_node = b - ab;
            if check_bounds(new_node, bounds) {
                antinode_map.insert(new_node);
            }
            let new_node = a + ab;
            if check_bounds(new_node, bounds) {
                antinode_map.insert(new_node);
            }
        }
    }
    //_print_map(bounds, map, &antinode_map);
    antinode_map.len() as i32
}

fn part_2((bounds, map): &(Bounds, HashMap<char, Vec<IVec2>>)) -> i32 {
    let mut antinode_map = HashSet::default();
    for antennas in map.values() {
        for x in antennas.iter().combinations(2) {
            let mut a = *x[0];
            let mut b = *x[1];
            let ab = a - b;
            antinode_map.insert(a);
            antinode_map.insert(b);
            loop {
                let new_node = b - ab;
                if check_bounds(new_node, bounds) {
                    antinode_map.insert(new_node);
                    b = new_node;
                } else {
                    break;
                }
            }
            loop {
                let new_node = a + ab;
                if check_bounds(new_node, bounds) {
                    antinode_map.insert(new_node);
                    a = new_node;
                } else {
                    break;
                }
            }
        }
    }
    //print_map(bounds, map, &antinode_map);
    antinode_map.len() as i32
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 14);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 34);
    }
}
