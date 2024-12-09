use std::{collections::HashMap, error};

use crate::inputs::read_lines;
use regex::{self, Regex};

struct Lists {
    left: Vec<u32>,
    right: Vec<u32>
}

fn parse_input(test: bool) -> Result<Lists, Box<dyn error::Error>> {
    let lines = read_lines(1, test)?;
    let re = Regex::new("(\\d+)   (\\d+)")?;
    let mut sortie = 
        Lists {
            left: Vec::new(),
            right: Vec::new(),
        };
    for l in lines {
        let numbers = re.captures(l.as_str()).unwrap();
        let left = numbers[1].parse::<u32>()?;
        let right = numbers[2].parse::<u32>()?;
        sortie.left.push(left); sortie.right.push(right);
    }
    Ok(sortie)
}

pub fn part1(test:bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    values.left.sort();
    values.right.sort();
    let mut sortie: u32 = 0;
    for i in 0..values.left.len() {
        sortie += values.left[i].abs_diff(values.right[i]);
    }
    Ok(sortie as u64)
}

pub fn part2(test:bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut similarities: HashMap<u32, u32> = HashMap::new();
    let mut sortie: u32 = 0;
    for i in values.right {
        *similarities.entry(i).or_insert(0) += 1;
    }
    for i in values.left {
        sortie += i * similarities.get(&i).unwrap_or(&0);
    }
    Ok(sortie as u64)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 11);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(),31);
}
