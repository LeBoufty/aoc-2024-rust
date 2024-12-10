use crate::chargrid;
use core::error;
use std::collections::{HashMap, HashSet};
use crate::inputs::read_lines;

struct TopoMap {
    levels: HashMap<u32, HashSet<(i32, i32)>>,
    width: i32,
    height: i32
}

impl TopoMap {
    fn next_possible(&self, positions: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
        let mut sortie: HashSet<(i32,i32)> = HashSet::new();
        let voisins = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for i in positions {
            for (dx, dy) in &voisins {
                let new_pos = (i.0 + dx, i.1 + dy);
                if self.in_bounds(new_pos) {
                    sortie.insert(new_pos);
                }
            }
        }
        sortie
    }
    fn next_possible_dupes(&self, positions: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut sortie: Vec<(i32,i32)> = Vec::new();
        let voisins = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for i in positions {
            for (dx, dy) in &voisins {
                let new_pos = (i.0 + dx, i.1 + dy);
                if self.in_bounds(new_pos) {
                    sortie.push(new_pos);
                }
            }
        }
        sortie
    }
    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        return pos.1 >= 0 && pos.0 >= 0
            && pos.1 < self.width && pos.0 < self.height;
    }
}

fn parse_input(test:bool) -> Result<TopoMap, Box<dyn error::Error>> {
    let lines = read_lines(10, test)?;
    let cg = chargrid::make_grid(&lines);
    let mut sortie: TopoMap = TopoMap{
        levels: HashMap::new(),
        width: cg.width() as i32,
        height: cg.height() as i32
    };
    for i in 0..10 {
        sortie.levels.entry(i)
            .or_insert(cg.find_all_hset(char::from_digit(i, 10).unwrap()));
    }
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let startpositions = values.levels.get(&0).unwrap().clone();
    let mut count = 0;
    for s in startpositions {
        let mut positions = HashSet::from([s]);
        for i in 0..9 {
            let nextpositions = values.next_possible(&positions)
                .intersection(values.levels.get(&(i+1)).unwrap())
                .cloned()
                .collect::<HashSet<(i32, i32)>>();
            positions = nextpositions;
        }
        count += positions.len() as u64;
    }
    Ok(count)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let startpositions = values.levels.get(&0).unwrap().clone();
    let mut count = 0;
    for s in startpositions {
        let mut positions = Vec::from([s]);
        for i in 0..9 {
            let nextpositions = values.next_possible_dupes(&positions).iter()
                .filter(|x| values.levels.get(&(i+1)).unwrap().contains(x))
                .cloned()
                .collect::<Vec<(i32, i32)>>();
            positions = nextpositions;
        }
        count += positions.len() as u64;
    }
    Ok(count)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 36);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 81);
}
