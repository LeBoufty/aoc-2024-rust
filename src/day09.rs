use core::error;
use itertools::Itertools;

use crate::inputs::read_input;

fn parse_input(test:bool) -> Result<Vec<String>, Box<dyn error::Error>> {
    let data = read_input(9, test)?;
    let mut id = 0;
    let mut empty = false;
    let mut sortie: Vec<String> = Vec::new();
    for i in data.chars().map(|x| x.to_digit(10).unwrap() as usize) {
        if empty {
            sortie.append(&mut vec![".".to_string(); i]);
            empty = false;
        } else {
            sortie.append(&mut vec![id.to_string(); i]);
            id += 1;
            empty = true;
        }
    }
    Ok(sortie)
}

fn find_first_empty(val: &Vec<String>) -> usize {
    for i in 0..val.len() {
        if val.get(i).unwrap().eq(&".") {
            return i
        }
    }
    0
}

fn find_last_full(val: &Vec<String>) -> usize {
    for i in 1..=val.len() {
        if !val.get(val.len() - i).unwrap().eq(&".") {
            return val.len() - i
        }
    }
    0
}

fn get_value(val: &Vec<String>) -> u64 {
    let mut sortie = 0;
    for i in 0..(find_last_full(val)+1) as u64 {
        sortie += i * val[i as usize].parse::<u64>().unwrap() as u64;
    }
    sortie
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    loop {
        let first_empty = find_first_empty(&values);
        let last_full = find_last_full(&values);
        if first_empty >= last_full {
            break;
        }
        values.swap(first_empty, last_full);
    }
    Ok(get_value(&values))
}

#[derive(Clone, Copy, Debug)]
struct Block {
    empty: bool,
    start: u64,
    size: u64,
    id: Option<u64>
}

#[derive(Debug)]
struct Disk {
    empty: Vec<Block>,
    full: Vec<Block>
}

fn build_disk(values: &Vec<String>) -> Disk {
    let mut sortie: Vec<Block> = Vec::new();
    let mut block = Block {
        empty: false,
        start: 0,
        size: 0,
        id: Some(0)
    };
    for i in 0..values.len() {
        if block.empty {
            if values[i].eq(&".") {
                block.size += 1;
            } else {
                sortie.push(block.clone());
                block = Block {
                    empty: false,
                    start: i as u64,
                    size: 1,
                    id: Some(values[i].parse::<u64>().unwrap())
                };
            }
        } else {
            if values[i].eq(&block.id.unwrap().to_string()) {
                block.size += 1;
            } else {
                sortie.push(block.clone());
                block = Block {
                    empty: values[i].eq(&"."),
                    start: i as u64,
                    size: 1,
                    id: values[i].parse::<u64>().ok()
                };
            }
        }
    }
    sortie.push(block.clone());
    Disk {
        empty: sortie.iter().filter(|b| b.empty).map(|b| b.clone()).collect_vec(),
        full: sortie.iter().filter(|b| !b.empty).map(|b| b.clone()).collect_vec()
    }
}

impl Disk {
    fn value(&self) -> u64 {
        let mut sum: u64 = 0;
        for b in &self.full {
            for i in b.start..b.start+b.size {
                sum += i * b.id.unwrap();
            }
        }
        sum
    }
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = build_disk(&parse_input(test)?);
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