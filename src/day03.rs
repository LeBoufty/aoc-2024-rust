use std::{error};
use regex::{self, Regex};
use crate::inputs::read_input;

struct Mul {
    left: i32,
    right: i32
}

impl Mul {
    fn value(&self) -> i32 {
        return self.left * self.right;
    }
}

fn parse_input(test:bool) -> Result<Vec<Mul>, Box<dyn error::Error>> {
    let data = read_input(3, test)?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let mut mules = vec![];
    for (_, [left, right]) in re.captures_iter(data.as_str()).map(|c| c.extract()) {
        mules.push(Mul {
            left: left.parse::<i32>()?,
            right: right.parse::<i32>()?
        });
    }
    return Ok(mules);
}

pub fn part1(test: bool) -> Result<i32, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut sortie = 0;
    for i in values {
        sortie += i.value();
    }
    return Ok(sortie);
}


#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 161);
}
