use crate::inputs::read_input;
use core::error;
use std::collections::HashMap;

struct Stones {
    stones: Vec<u64>,
    log: HashMap<(u64, usize), u64>
}

impl Stones {
    fn predict(&mut self, s: &u64, steps: usize) -> u64 {
        if self.log.contains_key(&(*s, steps)) {
            return self.log.get(&(*s, steps)).unwrap().clone();
        }
        let mut sortie: u64 = 0;
        if steps == 0 {return 1;}
        if s == &0 {sortie += self.predict(&1, steps-1);}
        else if s.to_string().len() % 2 == 0 {
            let mut tarte = s.to_string();
            sortie += self.predict(&tarte.split_off(tarte.len()/2).parse::<u64>().unwrap(), steps-1);
            sortie += self.predict(&tarte.parse::<u64>().unwrap(), steps-1);
        }
        else {sortie += self.predict(&(s*2024), steps-1)}
        self.log.entry((*s, steps)).or_insert(sortie);
        return sortie;
    }
}

fn parse_input(test:bool) -> Result<Stones, Box<dyn error::Error>> {
    let data = read_input(11, test)?;
    let stones = data.split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let sortie: Stones = Stones {stones, log: HashMap::new()};
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    Ok(values.stones.clone().iter().map(|x| values.predict(x, 25)).sum())
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    Ok(values.stones.clone().iter().map(|x| values.predict(x, 75)).sum())
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 55312);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 65601038650482);
}