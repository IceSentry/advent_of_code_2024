use glam::IVec2;
//use hashbrown::{HashMap, HashSet};
//use std::collections::{HashMap, HashSet};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;

struct Bounds {
    min: IVec2,
    max: IVec2,
}

//struct Bounds {
//    min_x: i32,
//    max_x: i32,
//    min_y: i32,
//    max_y: i32,
//}

fn main() {
    let input = std::fs::read_to_string("inputs/day06.txt").unwrap();
    let (map, bounds) = parse(&input);
    let result = part_1(&map, &bounds);
    println!("part_1: {result}");
    let result = part_2(&map, &bounds);
    println!("part_2: {result}");
}

fn parse(input: &str) -> (HashMap<IVec2, char>, Bounds) {
    let mut bounds = Bounds {
        min: IVec2::MAX,
        max: IVec2::MIN,
    };
    let mut map = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        bounds.min.y = (y as i32).min(bounds.min.y);
        bounds.max.y = (y as i32).max(bounds.max.y);
        for (x, c) in line.chars().enumerate() {
            bounds.min.x = (x as i32).min(bounds.min.x);
            bounds.max.x = (x as i32).max(bounds.max.x);
            if c != '.' {
                map.insert(IVec2::new(x as i32, y as i32), c);
            }
        }
    }
    //for y in bounds.min_y..=bounds.max_y {
    //    for x in bounds.min_x..=bounds.max_x {
    //        match map.get(&IVec2::new(x, y)) {
    //            Some(c) => print!("{c}"),
    //            None => print!("."),
    //        }
    //    }
    //    println!()
    //}
    (map, bounds)
}

fn simulate_path(
    map: &HashMap<IVec2, char>,
    bounds: &Bounds,
    guard_pos: IVec2,
) -> Option<HashSet<IVec2>> {
    let mut set = HashSet::default();
    let mut guard_pos = guard_pos;
    let mut dir = IVec2::new(0, -1);
    let mut cycle_tracker = HashSet::default();
    loop {
        let next_pos = guard_pos + dir;
        if cycle_tracker.contains(&(guard_pos, dir)) {
            return None;
        } else {
            cycle_tracker.insert((guard_pos, dir));
        }
        if let Some('#') = map.get(&next_pos) {
            // rotate
            dir = IVec2::new(-dir.y, dir.x);
            continue;
        }
        if guard_pos.x <= bounds.min.x || guard_pos.x >= bounds.max.x {
            break;
        }
        if guard_pos.y <= bounds.min.y || guard_pos.y >= bounds.max.y {
            break;
        }
        guard_pos = next_pos;
        set.insert(guard_pos);
    }
    //for y in bounds.min_y..=bounds.max_y {
    //    for x in bounds.min_x..=bounds.max_x {
    //        match (map.get(&IVec2::new(x, y)), set.get(&IVec2::new(x, y))) {
    //            (None, Some(_)) => print!("X"),
    //            (Some('#'), None) => print!("#"),
    //            (Some(_), Some(_)) => print!("#"),
    //            _ => print!("."),
    //        }
    //    }
    //    println!();
    //}
    Some(set)
}

fn part_1(map: &HashMap<IVec2, char>, bounds: &Bounds) -> i32 {
    let (guard_pos, _guard_char) = map.iter().find(|(_, v)| **v != '#').unwrap();
    simulate_path(map, bounds, *guard_pos).unwrap().len() as i32
}

fn part_2(map: &HashMap<IVec2, char>, bounds: &Bounds) -> i32 {
    let (guard_pos, _guard_char) = map.iter().find(|(_, v)| **v != '#').unwrap();
    let init_path = simulate_path(map, bounds, *guard_pos).unwrap();
    let mut cycles = 0;
    for pos in init_path {
        if pos == *guard_pos {
            continue;
        }
        let mut map = map.clone();
        map.insert(pos, '#');
        if simulate_path(&map, bounds, *guard_pos).is_none() {
            cycles += 1;
        }
    }
    cycles
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    pub fn part_1() {
        let (map, bounds) = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&map, &bounds);
        assert_eq!(result, 41);
    }

    #[test]
    pub fn part_2() {
        let (map, bounds) = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&map, &bounds);
        assert_eq!(result, 6);
    }
}
