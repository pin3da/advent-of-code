use input_parsing::read_input;
use std::cmp::min;

#[derive(Debug, Clone, Copy)]
struct Block {
    start: i32, // inclusive
    end: i32,   // exclusive
    id: i32,
    used: i32,
}

impl Block {
    fn free_space(&self) -> i32 {
        if !self.can_store() {
            return 0;
        }
        self.end - self.start - self.used
    }
    fn can_store(&self) -> bool {
        self.id == -1
    }
    fn real_start(&self) -> i32 {
        assert!(self.can_store());
        self.start + self.used
    }
    // only for non-empty blocks
    fn len(&self) -> i32 {
        if self.can_store() {
            return 0;
        }
        self.end - self.start
    }
}

fn main() {
    let input = read_input("./src/example.txt");
    let disk_map = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let has_files = |index: usize| index & 1 == 0;
    let mut disk = vec![];
    let mut id = 0;
    let mut start = 0;
    for i in 0..disk_map.len() {
        let end = start + disk_map[i] as i32;
        if has_files(i) {
            disk.push(Block {
                start,
                end,
                id: id,
                used: 0,
            });
            id += 1;
        } else {
            disk.push(Block {
                start,
                end,
                id: -1,
                used: 0,
            });
        }
        start = end;
    }

    part1(disk.clone());
    part2(disk.clone());
}

fn part1(disk: Vec<Block>) {
    let mut disk = disk;
    let mut start = 0;
    let mut end = disk.len() - 1;
    let mut sorted_disk = vec![];
    while start < end {
        // skip ending on empty block
        while disk[end].can_store() || disk[end].len() == 0 {
            end -= 1;
        }
        // skip blocks with ids
        while start < end && (!disk[start].can_store() || disk[start].free_space() <= 0) {
            sorted_disk.push(disk[start]);
            start += 1;
        }

        if start < end {
            assert!(disk[start].free_space() > 0);
            let capacity = min(disk[start].free_space(), disk[end].len());
            sorted_disk.push(Block {
                start: disk[start].real_start(),
                end: disk[start].real_start() + capacity as i32,
                id: disk[end].id,
                used: 0,
            });
            disk[start].used += capacity;
            if disk[start].free_space() == 0 {
                start += 1;
            }

            disk[end].end -= capacity;
        }
    }
    if start == end && disk[start].len() > 0 {
        sorted_disk.push(disk[start]);
    }

    println!("Part 1: {}", checksum(&sorted_disk));
}

fn checksum(disk: &Vec<Block>) -> i64 {
    for (cur, next) in disk.iter().zip(disk.iter().skip(1)) {
        assert!(cur.end == next.start);
    }
    let mut checksum: i64 = 0;
    for block in disk {
        if block.id == -1 {
            continue;
        }
        for i in block.start..block.end {
            checksum += (block.id as i64) * (i as i64);
        }
    }
    println!();
    checksum
}

// This would also work for part 1 if all the blocks are splitted into 1-length blocks.
fn part2(disk: Vec<Block>) {
    let mut disk = disk;
    let mut end_it = disk.len() - 1;

    while end_it > 0 {
        // skip empty blocks
        if disk[end_it].can_store() {
            end_it -= 1;
            continue;
        }
        let mut found = false;
        for start in 0..end_it {
            if disk[start].can_store() && disk[start].free_space() >= disk[end_it].len() {
                let prev_end = disk[start].end;
                let new_end = disk[start].start + disk[end_it].len();
                disk[start].id = disk[end_it].id;
                disk[start].end = new_end;
                disk[end_it].id = -1;
                if new_end < prev_end {
                    disk.insert(
                        start + 1,
                        Block {
                            start: new_end,
                            end: prev_end,
                            id: -1,
                            used: 0,
                        },
                    );
                    // shifted all by 1
                    end_it += 1;
                }
                found = true;
                break;
            }
        }
        if !found {
            end_it -= 1;
        }
    }
    println!("Part 2: {}", checksum(&disk));
}
