use core::error;
use std::collections::HashMap;
use crate::inputs::read_lines;

struct Ordering {
    before: HashMap<u32, Vec<u32>>,
    after: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>
}

impl Ordering {
    fn is_correct(&self, update: &Vec<u32>) -> bool {
        for i in 0..update.len()-1 {
            let befores_o = self.before.get(&update[i]);
            if !befores_o.is_none() {
                let befores = befores_o.unwrap();
                for j in i+1..update.len() {
                    if befores.contains(&update[j]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn get_problem(&self, update:&Vec<u32>) -> (usize, usize) {
        for i in 0..update.len()-1 {
            let befores_o = self.before.get(&update[i]);
            if !befores_o.is_none() {
                let befores = befores_o.unwrap();
                for j in i+1..update.len() {
                    if befores.contains(&update[j]) {
                        return (i, j);
                    }
                }
            }
        }
        (0, 0)
    }

    fn get_correct(&self) -> Vec<Vec<u32>> {
        let mut sortie: Vec<Vec<u32>> = Vec::new();
        self.updates.iter().for_each(|u| {
            if self.is_correct(u) {
                sortie.push(u.clone());
            }
        });
        sortie
    }

    fn get_incorrect(&self) -> Vec<Vec<u32>> {
        let mut sortie: Vec<Vec<u32>> = Vec::new();
        self.updates.iter().for_each(|u| {
            if !self.is_correct(u) {
                sortie.push(u.clone());
            }
        });
        sortie
    }

    fn corrected(&self, update: &Vec<u32>) -> Vec<u32> {
        let mut sortie = update.clone();
        while !self.is_correct(&sortie) {
            let indexes = self.get_problem(&sortie);
            sortie.swap(indexes.0, indexes.1);
        }
        sortie
    }
}

fn parse_input(test:bool) -> Result<Ordering, Box<dyn error::Error>> {
    let lines = read_lines(5, test)?;
    let mut sortie = Ordering {
        before: HashMap::new(),
        after: HashMap::new(),
        updates: Vec::new()
    };
    let mut parsing_rules = true;
    for l in lines {
        if l.eq("") {
            parsing_rules = false;
        } else if parsing_rules {
            let digits: Vec<u32> = l.split('|').map(|s| s.parse().unwrap()).collect();
            sortie.after.entry(digits[0]).or_insert(Vec::new()).push(digits[1]);
            sortie.before.entry(digits[1]).or_insert(Vec::new()).push(digits[0]);
        } else {
            let digits: Vec<u32> = l.split(',').map(|s| s.parse().unwrap()).collect();
            sortie.updates.push(digits.clone());
        }
    }
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u32, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let correct = values.get_correct();
    let mut sortie: u32 = 0;
    for u in correct {
        sortie += u.get(u.len()/2).unwrap();
    }
    Ok(sortie)
}

pub fn part2(test: bool) -> Result<u32, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let updates = values.get_incorrect();
    let mut sortie: u32 = 0;
    for u in updates {
        let corrected = values.corrected(&u);
        sortie += corrected.get(corrected.len()/2).unwrap();
    }
    Ok(sortie)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 143);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 123);
}