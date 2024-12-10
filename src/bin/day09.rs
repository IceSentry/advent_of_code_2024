use core::panic;

type Data = DiskMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day09.txt").unwrap();
    let parsed_input = parse(&input);
    let result = part_1(&parsed_input);
    println!("part_1: {result}");
    let result = part_2(&parsed_input);
    println!("part_2: {result}");
}

#[derive(Debug, Clone, Copy)]
enum DiskMap {
    FileLen(u32, u32),
    FreeSpace(u32),
}

fn parse(input: &str) -> Vec<Data> {
    let mut disk_map = Vec::new();
    let mut file_id = 0;
    for chunk in input.trim().chars().collect::<Vec<_>>().chunks(2) {
        if let [file_len, free_space] = chunk {
            disk_map.push(DiskMap::FileLen(file_id, file_len.to_digit(10).unwrap()));
            file_id += 1;
            disk_map.push(DiskMap::FreeSpace(free_space.to_digit(10).unwrap()));
        } else if let [file_len] = chunk {
            disk_map.push(DiskMap::FileLen(file_id, file_len.to_digit(10).unwrap()));
            file_id += 1;
        } else {
            panic!("invalid chunk {disk_map:?}");
        }
    }
    disk_map
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Block {
    FreeSpace,
    File(u64),
}
impl Block {
    fn is_file(&self) -> bool {
        match self {
            Block::FreeSpace => false,
            Block::File(_) => true,
        }
    }
}

fn _print_disk(blocks: &Vec<Block>) {
    for block in blocks {
        match block {
            Block::FreeSpace => print!("."),
            Block::File(id) => print!("{id}"),
        }
    }
    println!();
}

fn part_1(disk_map: &[Data]) -> u64 {
    let mut blocks = Vec::new();
    for block in disk_map {
        match block {
            DiskMap::FileLen(id, len) => {
                for _ in 0..*len {
                    blocks.push(Block::File(*id as u64));
                }
            }
            DiskMap::FreeSpace(len) => {
                for _ in 0..*len {
                    blocks.push(Block::FreeSpace)
                }
            }
        }
    }

    //print_disk(&blocks);

    let mut first_free_index = 0;
    let mut last_file_index = blocks.len() - 1;
    loop {
        while blocks[first_free_index] != Block::FreeSpace {
            first_free_index += 1;
        }
        if first_free_index >= last_file_index {
            break;
        }
        blocks.swap(first_free_index, last_file_index);
        last_file_index -= 1;
        while !blocks[last_file_index].is_file() {
            last_file_index -= 1;
        }
        //print_disk(&blocks);
    }
    let mut checksum = 0;
    for (i, block) in blocks.iter().enumerate() {
        match block {
            Block::FreeSpace => break,
            Block::File(id) => checksum += i as u64 * *id,
        }
    }
    checksum
}

fn find_free_space(disk_map: &[Data], file_size: u64) -> Option<usize> {
    let mut free_index = 0;
    while free_index < disk_map.len() - 1 {
        if let DiskMap::FreeSpace(size) = disk_map[free_index] {
            if size as u64 >= file_size {
                return Some(free_index);
            }
        }
        free_index += 1;
    }
    None
}

fn part_2(disk_map: &[Data]) -> u64 {
    fn _print_disk(blocks: &Vec<DiskMap>) {
        for block in blocks {
            match block {
                DiskMap::FileLen(id, len) => {
                    for _ in 0..*len {
                        print!("{id}");
                    }
                }
                DiskMap::FreeSpace(len) => {
                    for _ in 0..*len {
                        print!(".");
                    }
                }
            }
        }
        println!();
    }

    let mut blocks = disk_map.to_vec();
    let mut last_file_index = blocks.len() - 1;
    //print_disk(&blocks);
    while last_file_index > 0 {
        let DiskMap::FileLen(id, file_size) = disk_map[last_file_index] else {
            last_file_index -= 1;
            continue;
        };
        let Some(free_index) = find_free_space(&blocks, file_size as u64) else {
            last_file_index -= 1;
            continue;
        };
        let file_index = blocks
            .iter()
            .position(|block| match block {
                DiskMap::FileLen(id_b, _) => *id_b == id,
                DiskMap::FreeSpace(_) => false,
            })
            .unwrap();
        if free_index > file_index {
            last_file_index -= 1;
            continue;
        }
        let DiskMap::FreeSpace(ref mut size) = blocks[free_index] else {
            unreachable!("free index should be free space")
        };
        *size -= file_size;
        blocks[file_index] = DiskMap::FreeSpace(file_size);
        blocks.insert(free_index, DiskMap::FileLen(id, file_size));
        //print_disk(&blocks);
        last_file_index -= 1;
    }

    let mut checksum = 0;
    let mut i = 0;
    for block in blocks {
        match block {
            DiskMap::FileLen(id, len) => {
                for _ in 0..len {
                    checksum += i as u64 * id as u64;
                    i += 1;
                }
            }
            DiskMap::FreeSpace(len) => {
                i += len;
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
2333133121414131402
";

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_1(&input);
        assert_eq!(result, 1928);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUT.trim_start().trim_end());
        let result = super::part_2(&input);
        assert_eq!(result, 2858);
    }
}
