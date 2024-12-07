use crate::chargrid;
use crate::inputs::read_lines;
use std::error;
use itertools::Itertools;

#[derive(PartialEq, Clone)]
struct Guard {
    position: (i32, i32),
    facing: (i32, i32)
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    blocked: Vec<(i32, i32)>,
    guard: Guard
}

impl Map {
    fn is_in_grid(&self, l: usize, c: usize) -> bool {
        return l < self.height && c < self.width;
    }

    fn is_blocked(&self, coords: (i32, i32)) -> bool {
        return self.blocked.contains(&coords);
    }

    fn update(&mut self) -> Option<(i32, i32)> {
        let looking_at = (
            self.guard.position.0 + self.guard.facing.0,
            self.guard.position.1 + self.guard.facing.1
        );
        if looking_at.0 < 0 || looking_at.1 < 0 {
            return None;
        }
        if !self.is_in_grid(looking_at.0 as usize, looking_at.1 as usize) {
            return None;
        }
        if self.is_blocked(looking_at) {
            self.guard.facing = self.guard.rotate();
            return Some(self.guard.position.clone());
        } else {
            self.guard.position = looking_at;
            return Some(looking_at);
        }
    }

    fn get_next(&self) -> Option<Guard> {
        let mut copy = self.clone();
        let next = copy.update();
        if next.is_none() {return None;}
        else {
            return Some(
                Guard {
                    position: next.unwrap(),
                    facing: copy.guard.facing
                }
            )
        }
    }

    fn is_in_loop(&self) -> bool {
        let mut copy = self.clone();
        let mut explored: Vec<Guard> = Vec::new();
        while let Some(g) = copy.get_next() {
            if explored.contains(&g) {
                break;
            }
            explored.push(g.clone());
            copy.update();
        }
        return !copy.get_next().is_none();
    }

    fn obstruct(&mut self, coords: (i32, i32)) {
        self.blocked.push(coords);
    }
}


impl Guard {
    fn rotate(&mut self) -> (i32, i32) {
        let mut sortie = (0,0);
        if self.facing.0 == 0 {
            sortie.0 = self.facing.1;
            sortie.1 = 0;
        } else {
            sortie.1 = -self.facing.0;
            sortie.0 = 0;
        }
        sortie
    }
}

fn parse_input(test: bool) -> Result<Map, Box<dyn error::Error>> {
    let lines = read_lines(6, test)?;
    let cg = chargrid::make_grid(&lines);
    let g = Guard {
        position: {
            let &(x, y) = cg.find_all('^').first().unwrap();
            (x, y)
        },
        facing: (-1, 0)
    };
    return Ok(
        Map {
            width: cg.width(),
            height: cg.height(),
            blocked: cg.find_all('#'),
            guard: g
        }
    );
}

pub fn part1(test: bool) -> Result<u32, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    let mut explored: Vec<(i32, i32)> = vec![values.guard.position];
    while let Some(pos) = values.update() {
        explored.push(pos);
    }
    return Ok(explored.iter().unique().count() as u32);
}

pub fn part2(test: bool) -> Result<u32, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    let copy = values.clone();
    let mut obstructable = 0;
    let mut explored: Vec<(i32, i32)> = Vec::new();
    while let Some(pos) = values.update() {
        explored.push(pos);
    }
    explored = explored.iter().cloned().unique().collect();
    for i in explored {
        let mut obstructed = copy.clone();
        obstructed.obstruct(i);
        if obstructed.is_in_loop() {
            obstructable += 1;
        }
    }
    return Ok(obstructable);
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 41);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(),6);
}
