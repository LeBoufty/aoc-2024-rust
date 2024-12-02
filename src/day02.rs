use std::error;

use crate::inputs::read_lines;
use regex::{self, Regex};

struct Levels {
    values: Vec<i32>,
    rising: bool
}

impl Levels {
    fn is_safe(&self) -> bool {
        let t_min: i32;
        let t_max: i32;
        if self.rising {
            t_min = 1;
            t_max = 3
        } else {
            t_min = -3;
            t_max = -1;
        }
        for i in 0..self.values.len()-1 {
            let a = self.values[i];
            let b = self.values[i+1];
            if b - a > t_max
            || b - a < t_min {
                return false;
            }
        }
        true
    }
}

struct Reports {
    values: Vec<Levels>
}

fn parse_input(test:bool) -> Result<Reports, Box<dyn error::Error>> {
    let lines = read_lines(2, test)?;
    let mut sortie= Reports {
        values: Vec::new()
    };
    for l in lines {
        let mut rising = false;
        let digits: Vec<i32> = l.split(" ").map(|s| s.parse().unwrap()).collect();
        if digits.first() <= digits.last() {
            rising = true;
        }
        sortie.values.push(Levels {
            values: digits,
            rising: rising
        })
    }
    Ok(sortie)
}

pub fn part1(test:bool) -> Result<u32, Box<dyn error::Error>> {
    let mut sortie = 0;
    let reports = parse_input(test)?;
    for i in reports.values {
        if i.is_safe() {sortie += 1;}
    }
    Ok(sortie)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 2);
}