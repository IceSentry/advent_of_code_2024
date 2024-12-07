use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use glam::IVec2;

struct Bounds {
    min: IVec2,
    max: IVec2,
}

fn main() {
    let input = std::fs::read_to_string("inputs/day06.txt").unwrap();
    let (map, bounds) = parse(&input);
    let result = part_1(&map, &bounds);
    println!("part_1: {result}");
    let start = std::time::Instant::now();
    let result = part_2(&map, &bounds);
    let elapsed = start.elapsed().as_millis();
    println!("part_2: {result} {elapsed}ms");
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
    (map, bounds)
}

fn check_bounds(pos: IVec2, bounds: &Bounds) -> bool {
    if pos.x <= bounds.min.x || pos.x >= bounds.max.x {
        return false;
    }
    if pos.y <= bounds.min.y || pos.y >= bounds.max.y {
        return false;
    }
    true
}

fn rotate_dir(dir: IVec2) -> IVec2 {
    IVec2::new(-dir.y, dir.x)
}

fn simulate_path(map: &HashMap<IVec2, char>, bounds: &Bounds, guard_pos: IVec2) -> HashSet<IVec2> {
    let mut set = HashSet::default();
    let mut guard_pos = guard_pos;
    // guard always starts looking up`
    let mut dir = IVec2::new(0, -1);
    loop {
        let next_pos = guard_pos + dir;
        if let Some('#') = map.get(&next_pos) {
            dir = rotate_dir(dir);
            continue;
        }
        if !check_bounds(guard_pos, bounds) {
            break;
        }
        guard_pos = next_pos;
        set.insert(guard_pos);
    }
    //print_path(map, bounds, &set);
    set
}

fn check_cycles(
    map: &HashMap<IVec2, char>,
    bounds: &Bounds,
    guard_pos: IVec2,
    cycle_tracker: &mut HashSet<(IVec2, IVec2)>,
) -> bool {
    let mut guard_pos = guard_pos;
    let mut dir = IVec2::new(0, -1);
    cycle_tracker.clear();
    loop {
        let next_pos = guard_pos + dir;
        if cycle_tracker.contains(&(guard_pos, dir)) {
            return true;
        } else {
            cycle_tracker.insert((guard_pos, dir));
        }
        if let Some('#') = map.get(&next_pos) {
            dir = rotate_dir(dir);
            continue;
        }
        if !check_bounds(guard_pos, bounds) {
            break;
        }
        guard_pos = next_pos;
    }
    false
}

fn part_1(map: &HashMap<IVec2, char>, bounds: &Bounds) -> i32 {
    let (guard_pos, _guard_char) = map.iter().find(|(_, v)| **v != '#').unwrap();
    simulate_path(map, bounds, *guard_pos).len() as i32
}

fn part_2(map: &HashMap<IVec2, char>, bounds: &Bounds) -> i32 {
    let (guard_pos, _guard_char) = map.iter().find(|(_, v)| **v != '#').unwrap();
    let init_path = simulate_path(map, bounds, *guard_pos);
    let mut cycles = 0;
    let mut cycle_tracker = HashSet::default();
    for pos in init_path {
        if pos == *guard_pos {
            continue;
        }
        let mut map = map.clone();
        map.insert(pos, '#');
        if check_cycles(&map, bounds, *guard_pos, &mut cycle_tracker) {
            cycles += 1;
        }
    }
    cycles
}

fn _print_path(map: &HashMap<IVec2, char>, bounds: &Bounds, path: &HashSet<IVec2>) {
    for y in bounds.min.y..=bounds.max.y {
        for x in bounds.min.x..=bounds.max.x {
            match (map.get(&IVec2::new(x, y)), path.get(&IVec2::new(x, y))) {
                (None, Some(_)) => print!("X"),
                (Some('#'), None) => print!("#"),
                (Some(_), Some(_)) => print!("#"),
                _ => print!("."),
            }
        }
        println!();
    }
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
