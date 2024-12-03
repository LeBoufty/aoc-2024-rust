use std::{error};
use regex::{self, Regex};
use crate::inputs::read_input;

struct Mul {
    left: i32,
    right: i32,
    todo: bool
}

impl Mul {
    fn value(&self) -> i32 {
        return self.left * self.right;
    }
}

fn parse_input(test:bool) -> Result<Vec<Mul>, Box<dyn error::Error>> {
    let data = read_input(3, test)?;
    let re = Regex::new(r"((do\(\)))|((don't\(\)))|mul\((\d{1,3}),(\d{1,3})\)")?;
    let mut todo = true;
    let mut mules = vec![];
    for (token, [left, right]) in re.captures_iter(data.as_str()).map(|c| c.extract()) {
        if token.eq("do()") {todo = true;}
        else if token.eq("don't()") {todo = false;}
        else {
            mules.push(Mul {
                left: left.parse::<i32>()?,
                right: right.parse::<i32>()?,
                todo: todo.clone()
            });
        }
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

pub fn part2(test: bool) -> Result<i32, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut sortie = 0;
    for i in values {
        if i.todo {sortie += i.value();}
    }
    return Ok(sortie);
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 161);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 48);
}
