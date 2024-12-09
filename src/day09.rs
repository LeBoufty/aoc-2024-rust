use core::error;

use crate::inputs::read_input;
use std::cmp::min;

struct Block {
    start: u64,
    size: u64,
    id: u64
}

struct Disk {
    empty: Vec<Block>,
    full: Vec<Block>
}

impl Disk {
    fn value(&self) -> u64 {
        let mut sum: u64 = 0;
        for b in &self.full {
            sum += (b.start..b.start+b.size)
                .map(|i| i * b.id).sum::<u64>();
        }
        sum
    }
}

fn parse_input(test:bool) -> Result<Disk, Box<dyn error::Error>> {
    let data = read_input(9, test)?;
    let mut id = 0;
    let mut start = 0;
    let mut empty = false;
    let mut sortie: Disk = Disk {
        empty: Vec::new(),
        full: Vec::new()
    };
    for size in data.chars().map(|x| x.to_digit(10).unwrap() as u64) {
        if empty {
            sortie.empty.push(Block {
                start,
                size,
                id: 0
            });
            id += 1;
        } else {
            sortie.full.push(Block {
                start,
                size,
                id
            });
        }
        empty = !empty;
        start += size;
    }
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    values.full.reverse();
    let mut nextfull: Vec<Block> = Vec::new();
    for f in &mut values.full {
        for e in &mut values.empty {
            if f.size > 0 && e.size > 0 && f.start > e.start {
                let transfer = min(f.size, e.size);
                let new_block = Block {
                    start: e.start,
                    size: transfer,
                    id: f.id
                };
                f.size -= transfer;
                e.size -= transfer;
                e.start += transfer;
                nextfull.push(new_block);
            }
        }
    }
    values.full.append(&mut nextfull);
    Ok(values.value())
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    values.full.reverse();
    for f in &mut values.full {
        for e in &mut values.empty {
            if f.size <= e.size && f.start > e.start {
                f.start = e.start;
                e.size -= f.size;
                e.start += f.size;
            }
        }
    }
    Ok(values.value())
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 1928);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 2858);
}