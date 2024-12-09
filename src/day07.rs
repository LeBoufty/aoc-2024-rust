use crate::inputs::read_lines;
use std::error;
use radix_fmt::radix_3;
use rayon::prelude::*;

struct Equation {
    result: u64,
    numbers: Vec<u64>
}

impl Equation {
    fn is_possible(&self) -> bool {
        let op_nb = self.numbers.len()-1;
        for i in 0..2_i32.pow(op_nb as u32) {
            let mut operators: Vec<char> = format!("{i:032b}").chars().collect();
            operators = operators.split_at(32 - op_nb).1.to_vec();
            if self.evaluate(operators) == self.result {return true}
        }
        false
    }

    fn is_possible_concat(&self) -> bool {
        let op_nb = self.numbers.len()-1;
        for i in 0..3_i32.pow(op_nb as u32) {
            let mut operators: Vec<char> = radix_3(i).to_string().chars().collect();
            while operators.len() < op_nb {
                operators.insert(0, '0');
            }
            if self.evaluate(operators) == self.result {return true}
        }
        false
    }

    fn evaluate(&self, operators: Vec<char>) -> u64 {
        let mut result = self.numbers[0];
        for i in 0..operators.len() {
            if operators[i] == '2' {
                result = format!("{}{}", result, self.numbers[i+1])
                    .parse::<u64>()
                    .unwrap();
            }
            else if operators[i] == '1' {result *= self.numbers[i+1]}
            else {result += self.numbers[i+1]}
        }
        result
    }
}

struct Calibration {
    equations: Vec<Equation>
}

fn parse_input(test: bool) -> Result<Calibration, Box<dyn error::Error>> {
    let lines = read_lines(7, test)?;
    let mut equations: Vec<Equation> = Vec::new();
    for l in lines {
        let sp_line: Vec<&str> = l.split(": ").collect();
        let result = sp_line[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = sp_line[1].split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        equations.push(Equation {result, numbers});
    }
    Ok(Calibration {equations})
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let sortie: u64 = values.equations.par_iter()
        .filter(|x| x.is_possible())
        .map(|x| x.result)
        .sum();
    Ok(sortie)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let sortie: u64 = values.equations.par_iter()
        .filter(|x| x.is_possible_concat())
        .map(|x| x.result)
        .sum();
    Ok(sortie)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 3749);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(),11387);
}
